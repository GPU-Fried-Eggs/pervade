use super::file_name_template::FileNameTemplate;

#[derive(Debug, Default)]
pub struct OutputOptions {
    pub entry_file_names: Option<String>,
    pub chunk_file_names: Option<String>,
    pub dir: Option<String>,
}

#[derive(Debug)]
pub struct InternalOutputOptions {
    pub entry_file_names: FileNameTemplate,
    pub chunk_file_names: FileNameTemplate,
}

impl From<OutputOptions> for InternalOutputOptions {
    fn from(value: OutputOptions) -> Self {
        Self {
            entry_file_names: FileNameTemplate::from(
                value
                    .entry_file_names
                    .unwrap_or_else(|| "[name].js".to_string()),
            ),
            chunk_file_names: FileNameTemplate::from(
                value
                    .chunk_file_names
                    .unwrap_or_else(|| "[name]-[hash].js".to_string()),
            ),
        }
    }
}
