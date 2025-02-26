mod visit;

use std::collections::HashMap;

use oxc::ast::ast;
use oxc::semantic::{ScopeTree, SymbolFlags, SymbolId, SymbolTable};
use oxc::span::{Atom, CompactStr};
use oxc_index::IndexVec;

use crate::common::{
    ImportRecord, ImportRecordId, LocalExport, LocalOrReExport, ModuleId, NamedImport, ReExport,
    Specifier, StmtInfo, StmtInfoId,
};

#[derive(Debug, Default)]
pub struct ScanResult {
    pub named_imports: HashMap<SymbolId, NamedImport>,
    pub named_exports: HashMap<CompactStr, LocalOrReExport>,
    pub stmt_infos: IndexVec<StmtInfoId, StmtInfo>,
    pub import_records: IndexVec<ImportRecordId, ImportRecord>,
    pub star_exports: Vec<ImportRecordId>,
    pub export_default_symbol_id: Option<SymbolId>,
}

pub struct Scanner<'a> {
    pub id: ModuleId,
    pub scope: &'a mut ScopeTree,
    pub symbol_table: &'a mut SymbolTable,
    pub current_stmt_info: StmtInfo,
    pub result: ScanResult,
    pub unique_name: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn new(
        id: ModuleId,
        scope: &'a mut ScopeTree,
        symbol_table: &'a mut SymbolTable,
        unique_name: &'a str,
    ) -> Self {
        Self {
            id,
            scope,
            symbol_table,
            current_stmt_info: Default::default(),
            result: Default::default(),
            unique_name,
        }
    }

    fn add_declared_id(&mut self, id: SymbolId) {
        self.current_stmt_info.declared_symbols.push(id);
    }

    fn get_root_binding(&self, name: CompactStr) -> SymbolId {
        self.scope
            .get_root_binding(name.as_str())
            .expect("must have")
    }

    fn add_import_record(&mut self, module_request: CompactStr) -> ImportRecordId {
        let rec = ImportRecord::new(module_request);
        let idx = self.result.import_records.push(rec);
        idx
    }

    fn add_named_import(&mut self, local: SymbolId, imported: CompactStr, record_id: ImportRecordId) {
        self.result.named_imports.insert(
            local,
            NamedImport {
                imported: Specifier::Literal(imported.to_owned()),
                imported_as: (self.id, local).into(),
                record_id,
            },
        );
    }

    fn add_star_import(&mut self, local: SymbolId, record_id: ImportRecordId) {
        self.result.import_records[record_id].is_import_namespace = true;
        self.result.named_imports.insert(
            local,
            NamedImport {
                imported: Specifier::Star,
                imported_as: (self.id, local).into(),
                record_id,
            },
        );
    }

    fn add_local_export(&mut self, export_name: &Atom, local: SymbolId) {
        self.result.named_exports.insert(
            export_name.to_owned().into(),
            LocalOrReExport::Local(LocalExport {
                referenced: (self.id, local).into(),
            }),
        );
    }

    fn add_local_default_export(&mut self, local: SymbolId) {
        self.result.export_default_symbol_id = Some(local);
        self.result.named_exports.insert(
            "default".into(),
            LocalOrReExport::Local(LocalExport {
                referenced: (self.id, local).into(),
            }),
        );
    }

    fn add_re_export(&mut self, export_name: &Atom, imported: &Atom, record_id: ImportRecordId) {
        self.result.named_exports.insert(
            export_name.to_owned().into(),
            LocalOrReExport::Re(ReExport {
                imported: Specifier::Literal(imported.to_owned().into()),
                record_id,
            }),
        );
    }

    fn add_star_re_export(&mut self, export_name: &Atom, record_id: ImportRecordId) {
        self.result.import_records[record_id].is_import_namespace = true;
        self.result.named_exports.insert(
            export_name.to_owned().into(),
            LocalOrReExport::Re(ReExport {
                imported: Specifier::Star,
                record_id,
            }),
        );
    }

    fn scan_export_all_decl(&mut self, decl: &ast::ExportAllDeclaration) {
        let id = self.add_import_record(decl.source.value.into());
        if let Some(exported) = &decl.exported {
            // export * as ns from '...'
            self.add_star_re_export(&exported.name(), id)
        } else {
            // export * from '...'
            self.result.star_exports.push(id);
        }
    }

    fn scan_export_named_decl(&mut self, decl: &ast::ExportNamedDeclaration) {
        if let Some(source) = &decl.source {
            let record_id = self.add_import_record(source.value.into());
            decl.specifiers.iter().for_each(|spec| {
                self.add_re_export(&spec.exported.name(), &spec.local.name(), record_id);
            })
        } else {
            decl.specifiers.iter().for_each(|spec| {
                self.add_local_export(
                    &spec.local.name(),
                    self.get_root_binding(spec.local.name().into()),
                );
            });
            if let Some(decl) = decl.declaration.as_ref() {
                match decl {
                    ast::Declaration::VariableDeclaration(var_decl) => var_decl
                        .declarations
                        .iter()
                        .for_each(|decl| match &decl.id.kind {
                            ast::BindingPatternKind::BindingIdentifier(id) => {
                                self.result.named_exports.insert(
                                    id.name.to_owned().into(),
                                    LocalExport {
                                        referenced: (self.id, id.symbol_id.get().unwrap()).into(),
                                    }
                                    .into(),
                                );
                            }
                            _ => unimplemented!(),
                        }),
                    ast::Declaration::FunctionDeclaration(fn_decl) => {
                        let id = fn_decl.id.as_ref().unwrap();
                        self.add_local_export(&id.name, id.symbol_id.get().unwrap());
                    }
                    ast::Declaration::ClassDeclaration(cls_decl) => {
                        let id = cls_decl.id.as_ref().unwrap();
                        self.add_local_export(&id.name, id.symbol_id.get().unwrap());
                    }
                    _ => unreachable!("doesn't support ts now"),
                }
            }
        }
    }

    fn get_symbol_id_from_identifier_reference(
        &self,
        id_ref: &ast::IdentifierReference,
    ) -> SymbolId {
        let ref_id = id_ref.reference_id.get().unwrap();
        let refer = self.symbol_table.get_reference(ref_id);
        refer.symbol_id().unwrap()
    }

    fn scan_export_default_decl(&mut self, decl: &ast::ExportDefaultDeclaration) {
        let local = match &decl.declaration {
            ast::ExportDefaultDeclarationKind::Identifier(id_ref) => {
                Some(self.get_symbol_id_from_identifier_reference(id_ref))
            }
            ast::ExportDefaultDeclarationKind::FunctionDeclaration(fn_decl) => {
                fn_decl.id.as_ref().map(|bind_id| bind_id.symbol_id.get().unwrap())
            },
            ast::ExportDefaultDeclarationKind::ClassDeclaration(cls_decl) => {
                cls_decl.id.as_ref().map(|bind_id| bind_id.symbol_id.get().unwrap())
            },
            ast::ExportDefaultDeclarationKind::TSInterfaceDeclaration(_) => unreachable!(),
            _ => None,
        };

        let local = local.unwrap_or_else(|| {
            // For patterns like `export default [expression]`, we need to create
            // a facade Symbol to represent it.
            // Notice: Patterns don't include `export default [identifier]`
            let root_scope_id = self.scope.root_scope_id();
            let sym_id = self.symbol_table.create_symbol(
                Default::default(),
                &format!("{}_default", self.unique_name),
                SymbolFlags::None,
                root_scope_id,
                self.scope.get_node_id(root_scope_id),
            );

            self.current_stmt_info.declared_symbols.push(sym_id);
            sym_id
        });
        self.add_local_default_export(local);
    }

    fn scan_import_decl(&mut self, decl: &ast::ImportDeclaration) {
        let id = self.add_import_record(decl.source.value.into());
        let Some(specifiers) = &decl.specifiers else {
            return;
        };
        specifiers.iter().for_each(|spec| match spec {
            ast::ImportDeclarationSpecifier::ImportSpecifier(spec) => {
                let sym = spec.local.symbol_id.get().unwrap();
                self.add_named_import(sym, spec.imported.name().into(), id);
            }
            ast::ImportDeclarationSpecifier::ImportDefaultSpecifier(spec) => {
                self.add_named_import(
                    spec.local.symbol_id.get().unwrap(),
                    CompactStr::new_const("default"),
                    id,
                );
            }
            ast::ImportDeclarationSpecifier::ImportNamespaceSpecifier(spec) => {
                self.add_star_import(spec.local.symbol_id.get().unwrap(), id);
            }
        });
    }

    fn scan_module_decl(&mut self, decl: &ast::ModuleDeclaration) {
        match decl {
            ast::ModuleDeclaration::ImportDeclaration(decl) => {
                self.scan_import_decl(decl);
            }
            ast::ModuleDeclaration::ExportAllDeclaration(decl) => {
                self.scan_export_all_decl(decl);
            }
            ast::ModuleDeclaration::ExportNamedDeclaration(decl) => {
                self.scan_export_named_decl(decl);
            }
            ast::ModuleDeclaration::ExportDefaultDeclaration(decl) => {
                self.scan_export_default_decl(decl)
            }
            _ => {}
        }
    }
}
