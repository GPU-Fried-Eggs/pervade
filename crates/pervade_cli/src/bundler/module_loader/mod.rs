mod module_task;
mod task_result;

use std::collections::HashMap;
use std::sync::Arc;

use oxc_index::IndexVec;
use task_result::TaskResult;

use crate::bundler::graph::Graph;
use crate::bundler::module::Module;
use crate::bundler::options::InputOptions;
use crate::bundler::resolve_id::ResolvedRequestInfo;
use crate::bundler::symbols::SymbolMap;
use crate::common::ModuleId;
use crate::error::Error;
use crate::resolver::{FileSystem, Resolver};

pub enum Msg {
    Done(TaskResult),
}

pub struct ModuleLoader<'a> {
    input_options: &'a InputOptions,
    graph: &'a mut Graph,
    resolver: Arc<Resolver>,
    visited: HashMap<String, ModuleId>,
    remaining: u32,
    tx: tokio::sync::mpsc::UnboundedSender<Msg>,
    rx: tokio::sync::mpsc::UnboundedReceiver<Msg>,
}

impl<'a> ModuleLoader<'a> {
    pub fn new(
        input_options: &'a InputOptions,
        graph: &'a mut Graph,
        resolver: Arc<Resolver>,
    ) -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Msg>();

        Self {
            tx,
            rx,
            input_options,
            resolver,
            visited: Default::default(),
            remaining: Default::default(),
            graph,
        }
    }

    pub async fn fetch_all_modules(&mut self) -> Result<(), Error> {
        if self.input_options.input.is_empty() {
            return Err(Error::InvalidConfig("You must supply options.input".into()));
        }

        let resolved_entries = self.resolve_entries().await?;

        let mut intermediate_modules: IndexVec<ModuleId, Option<Module>> =
            IndexVec::with_capacity(self.graph.entries.len());

        self.graph.entries = resolved_entries
            .into_iter()
            .map(|p| self.try_spawn_new_task(&p, &mut intermediate_modules))
            .collect();

        let mut tables: IndexVec<ModuleId, SymbolMap> = Default::default();

        while self.remaining > 0 {
            let Some(msg) = self.rx.recv().await else {
                break;
            };
            match msg {
                Msg::Done(task_result) => {
                    let TaskResult {
                        module_id,
                        symbol_map: symbol_table,
                        resolved_deps,
                        errors,
                        warnings,
                        mut builder,
                    } = task_result;

                    let import_records = builder.import_records.as_mut().unwrap();

                    resolved_deps
                        .into_iter()
                        .for_each(|(import_record_idx, info)| {
                            let id = self.try_spawn_new_task(&info, &mut intermediate_modules);
                            import_records[import_record_idx].resolved_module = id;
                            while tables.len() <= id.raw() as usize {
                                tables.push(Default::default());
                            }
                        });

                    while tables.len() <= task_result.module_id.raw() as usize {
                        tables.push(Default::default());
                    }
                    intermediate_modules[module_id] = Some(Module::Normal(builder.build()));

                    tables[task_result.module_id] = symbol_table
                }
            }
            self.remaining -= 1;
        }
        self.graph.symbols = Symbols::new(tables);
        self.graph.modules = intermediate_modules
            .into_iter()
            .map(|m| m.unwrap())
            .collect();

        Ok(())
    }

    async fn resolve_entries(&mut self) -> Result<Vec<ResolvedRequestInfo>, Error> {
        let resolver = &self.resolver;

        let resolved_ids = block_on_spawn_all(self.input_options.input.iter().map(
            |input_item| async move {
                let specifier = &input_item.import;
                let resolve_id = resolve_id(resolver, specifier, None, false).await.unwrap();

                let Some(info) = resolve_id else {
                    return Err(BuildError::unresolved_entry(specifier));
                };

                if info.is_external {
                    return Err(BuildError::entry_cannot_be_external(info.path.as_str()));
                }

                Ok(info)
            },
        ));

        let mut errors = vec![];

        Ok(vec![])
    }
}
