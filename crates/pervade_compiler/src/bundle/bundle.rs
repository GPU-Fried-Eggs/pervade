use crate::options::InternalOutputOptions;

pub struct Bundle<'a> {
    graph: &'a mut Graph,
    output_options: &'a InternalOutputOptions,
}

impl<'a> Bundle<'a> {
    pub fn new(graph: &'a mut Graph, output_options: &'a InternalOutputOptions) -> Self {
        Self {
            graph,
            output_options,
        }
    }
}
