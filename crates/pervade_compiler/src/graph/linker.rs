use super::graph::Graph;

pub struct Linker<'graph> {
    graph: &'graph mut Graph,
}

impl<'graph> Linker<'graph> {
    pub fn new(graph: &'graph mut Graph) -> Self {
        Self { graph }
    }

    pub fn link(&mut self) {
        // propagate star exports
        for id in &self.graph.sorted_modules {
            let importer = &self.graph.modules[*id];
            match importer {
                Module::Normal(importer) => {
                    let resolved = importer.resolve_star_exports(&self.graph.modules);
                    self.graph.modules[*id]
                        .expect_normal_mut()
                        .resolved_star_exports = resolved;
                }
                Module::External(_) => {
                    // meaningless
                }
            }
        }

        Self::mark_whether_namespace_referenced(self.graph);

        self.graph
            .sorted_modules
            .clone()
            .into_iter()
            .for_each(|id| {
                self.resolve_exports(id);
                self.resolve_imports(id);
            })
    }
}
