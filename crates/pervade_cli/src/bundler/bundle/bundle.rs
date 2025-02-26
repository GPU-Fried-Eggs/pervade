use crate::bundler::stages::Graph;
use crate::bundler::options::OutputOptions;

pub struct Bundle<'a> {
    graph: &'a mut Graph,
    output_options: &'a OutputOptions,
}
