use std::sync::Arc;

use oxc_index::IndexVec;

use crate::bundler::ast_scanner::ScanResult;
use crate::bundler::module::NormalModuleBuilder;
use crate::bundler::symbols::SymbolMap;
use crate::common::{ModuleId, ResourceId};
use crate::error::Error;
use crate::resolver::Resolver;

use super::task_result::TaskResult;
use super::Msg;

pub struct ModuleTask {
    module_id: ModuleId,
    path: ResourceId,
    tx: tokio::sync::mpsc::UnboundedSender<Msg>,
    errors: Vec<Error>,
    warnings: Vec<Error>,
    resolver: Arc<Resolver>,
}

impl ModuleTask {
    pub fn new(
        id: ModuleId,
        resolver: Arc<Resolver>,
        path: ResourceId,
        tx: tokio::sync::mpsc::UnboundedSender<Msg>,
    ) -> Self {
        Self {
            module_id: id,
            resolver,
            path,
            tx,
            errors: Default::default(),
            warnings: Default::default(),
        }
    }

    pub async fn run(mut self) -> Result<(), Error>{
        let mut builder = NormalModuleBuilder::default();

        let source = tokio::fs::read_to_string(self.path.as_ref()).await?;

        let (ast, scope, scan_result, symbol) = self.make_ast(source);

        let res = self
            .resolve_dependencies(&scan_result.import_records)
            .await?;

        let mut symbol_map = SymbolMap::from(symbol);

        let ScanResult {
            named_imports,
            named_exports,
            stmt_infos,
            import_records,
            star_exports,
            export_default_symbol_id,
        } = scan_result;

        builder.id = Some(self.module_id);
        builder.ast = Some(ast);
        builder.path = Some(self.path);
        builder.named_imports = Some(named_imports);
        builder.named_exports = Some(named_exports);
        builder.stmt_infos = Some(stmt_infos);
        builder.import_records = Some(import_records);
        builder.star_exports = Some(star_exports);
        builder.default_export_symbol = export_default_symbol_id;
        builder.scope = Some(scope);
        builder.initialize_namespace_binding(&mut symbol_map);

        self.tx
            .send(Msg::Done(TaskResult {
                resolved_deps: res,
                module_id: self.module_id,
                errors: self.errors,
                warnings: self.warnings,
                symbol_map,
                builder,
            }))
            .unwrap();
        Ok(())
    }

    fn make_ast(&self, source: String) -> (OxcProgram, ScopeTree, ScanResult, SymbolTable) {
        let source_type = SourceType::from_path(self.path.as_ref()).unwrap();
        let mut program = OxcCompiler::parse(source, source_type);

        let semantic = program.make_semantic();
        let (mut symbol_table, mut scope) = semantic.into_symbol_table_and_scope_tree();
        let unique_name = self.path.stabilize();
        let mut scanner =
            scanner::Scanner::new(self.module_id, &mut scope, &mut symbol_table, &unique_name);
        scanner.visit_program(program.program_mut());
        let scan_result = scanner.result;

        (program, scope, scan_result, symbol_table)
    }

    pub(crate) async fn resolve_id(
        resolver: &Resolver,
        importer: &ResourceId,
        specifier: &str,
    ) -> BuildResult<ResolvedRequestInfo> {
        // let is_marked_as_external = is_external(specifier, Some(importer.id()), false).await?;

        // if is_marked_as_external {
        //     return Ok(ModuleId::new(specifier, true));
        // }

        let resolved_id = resolve_id(resolver, specifier, Some(importer), false).await?;

        match resolved_id {
            Some(info) => Ok(info),
            None => {
                Ok(ResolvedRequestInfo {
                    path: specifier.to_string().into(),
                    is_external: true,
                })
                // // TODO: should emit warnings like https://rollupjs.org/guide/en#warning-treating-module-as-external-dependency
                // return Err(rolldown_error::Error::unresolved_import(
                //   specifier.to_string(),
                //   importer.prettify(),
                // ));
            }
        }
    }

    async fn resolve_dependencies(
        &mut self,
        dependencies: &IndexVec<ImportRecordId, ImportRecord>,
    ) -> anyhow::Result<Vec<(ImportRecordId, ResolvedRequestInfo)>> {
        let jobs = dependencies.iter_enumerated().map(|(idx, item)| {
            let specifier = item.module_request.clone();
            let resolver = self.resolver.clone();
            let importer = self.path.clone();
            // let is_external = self.is_external.clone();
            // let on_warn = self.input_options.on_warn.clone();
            tokio::spawn(async move {
                Self::resolve_id(&resolver, &importer, &specifier)
                    .await
                    .map(|id| (idx, id))
            })
        });

        let resolved_ids = join_all(jobs).await;

        let mut errors = vec![];

        let ret = resolved_ids
            .into_iter()
            .filter_map(|handle| match handle.unwrap() {
                Ok(item) => Some(item),
                Err(e) => {
                    errors.push(e);
                    None
                }
            })
            .collect();

        Ok(ret)
    }
}
