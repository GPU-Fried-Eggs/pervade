use std::path::PathBuf;

use pervade_compiler::{Bundler, InputOptions, OutputOptions};

#[tokio::main]
async fn main() {
    let root = PathBuf::from(&std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let cwd = root.join("./examples");
    let mut bundler = Bundler::new(InputOptions {
        input: Some(vec!["./index.js".to_string().into()]),
        cwd: Some(cwd),
    });

    bundler
        .generate(OutputOptions { ..Default::default() })
        .await
        .unwrap();
}
