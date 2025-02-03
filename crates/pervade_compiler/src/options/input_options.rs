use std::path::PathBuf;

#[derive(Debug)]
pub struct InputItem {
    pub name: Option<String>,
    pub import: String,
}

impl From<String> for InputItem {
    fn from(value: String) -> Self {
        Self {
            name: None,
            import: value,
        }
    }
}

#[derive(Debug, Default)]
pub struct InputOptions {
    pub input: Option<Vec<InputItem>>,
    pub cwd: Option<PathBuf>,
}

#[derive(Debug, Default)]
pub struct InternalInputOptions {
    pub input: Vec<InputItem>,
    pub cwd: PathBuf,
}

impl From<InputOptions> for InternalInputOptions {
    fn from(value: InputOptions) -> Self {
        Self {
            input: value.input.unwrap_or_default(),
            cwd: value.cwd.unwrap_or_else(|| std::env::current_dir().unwrap()),
        }
    }
}
