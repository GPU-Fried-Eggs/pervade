use std::error::Error;

pub trait Runtime {
    fn load_image(&mut self, image_bytes: &[u8]) -> Result<(), Box<dyn Error>>;

    fn compile(&mut self, source_code: &[u8]) -> Result<Vec<u8>, Box<dyn Error>>;
}
