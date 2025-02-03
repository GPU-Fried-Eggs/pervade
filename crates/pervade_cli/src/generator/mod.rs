mod exports;
mod runtime;

use std::{error::Error, path::Path};
use std::rc::Rc;

pub use runtime::Runtime;

pub enum CodeGenType {
    Static,
    Dynamic,
}

pub struct Generator<'a, T: Runtime> {
    source_code: Rc<String>,
    runtime: &'a mut T,
}

impl<'a, T: Runtime> Generator<'a, T> {
    pub fn new(source_code: String, runtime: &'a mut T) -> Self {
        Self {
            source_code: Rc::new(source_code),
            runtime,
        }
    }

    pub fn load_image(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        let bytes = std::fs::read(path)?;
        self.runtime.load_image(&bytes)
    }

    pub fn compile(&mut self, js_source_code: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        self.runtime.compile(js_source_code)
    }

    pub fn exports(&self) -> Result<Vec<String>, Box<dyn Error>> {
        Ok(vec![])
    }
}
