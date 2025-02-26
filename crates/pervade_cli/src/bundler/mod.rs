mod ast_scanner;
//mod bundle;
mod graph;
mod module;
mod module_finalizer;
mod module_loader;
mod options;
mod resolve_id;
mod symbols;

use options::InputOptions;

pub struct Bundler {
    input_options: InputOptions,
}

impl Bundler {
    pub fn new(input_options: InputOptions) -> Self {
        Self { input_options }
    }
}
