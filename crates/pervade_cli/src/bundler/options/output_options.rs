use super::FileNameTemplate;

pub struct OutputOptions {
    pub entry_file_names: FileNameTemplate,
    pub chunk_file_names: FileNameTemplate,
}

impl Default for OutputOptions {
    fn default() -> Self {
        Self {
            entry_file_names: FileNameTemplate::from("[name].js".to_string()),
            chunk_file_names: FileNameTemplate::from("[name]-[hash].js".to_string()),
        }
    }
}
