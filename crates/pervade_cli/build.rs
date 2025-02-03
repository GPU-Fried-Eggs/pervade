use std::{env, error::Error, fs, path::PathBuf, process::Command};

fn main() {
    if let Err(e) = copy_plugin() {
        eprintln!("Prepare runtime failed: {}", e);
        std::process::exit(1);
    }
}

fn should_skip_build() -> bool {
    // skipping when `cargo check` or `cargo clippy`
    env::var("CARGO_CFG_TARGET_ARCH").is_err()
}

fn copy_plugin() -> Result<(), Box<dyn Error>> {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let out_dir = env::var("OUT_DIR")?;

    let workspace_dir = PathBuf::from(&cargo_manifest_dir).join("../..");
    let runtime_path = workspace_dir.join("target/wasm32-wasip1/release/pervade_runtime.wasm");

    let copied_runtime_path = PathBuf::from(&out_dir).join("pervade_runtime.wasm");

    if !should_skip_build() {
        let status = Command::new("cargo")
            .args(["build", "--release", "--package", "pervade_runtime", "--target", "wasm32-wasip1"])
            .current_dir(&workspace_dir)
            .status()?;

        if !status.success() {
            return Err("Failed to build runtime package.".into());
        }
    }

    if runtime_path.exists() {
        fs::copy(&runtime_path, &copied_runtime_path)?;
    } else {
        fs::write(&copied_runtime_path, [])?;
    }

    Ok(())
}
