use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(r#"Could not resolve entry module "{}"."#, unresolved_id.display())]
    UnresolvedEntry {
        unresolved_id: PathBuf,
    },

    #[error(r#"Entry module "{}" cannot be external."#, id.display())]
    ExternalEntry {
        id: PathBuf,
    },

    #[error(r#""{missing_export}" is not exported by "{}", imported by "{}"."#, importee.display(), importer.display())]
    MissingExport {
        importer: PathBuf,
        importee: PathBuf,
        missing_export: String,
    },

    #[error(
        r#"Ambiguous external namespace resolution: "{}" re-exports "{binding}" from one of the external modules {}, guessing "{}"."#,
        reexporting_module.display(),
        format_quoted_strings(&sources.iter().map(|p| p.display().to_string()).collect::<Vec<_>>()),
        used_module.display()
    )]
    AmbiguousExternalNamespaces {
        reexporting_module: PathBuf,
        used_module: PathBuf,
        binding: String,
        sources: Vec<PathBuf>,
    },

    #[error(r#"Circular dependency: {}."#, format_paths(&.0))]
    CircularDependency(Vec<PathBuf>),

    #[error(r#""output.exports" must be "default", "named", "none", "auto", or left unspecified (defaults to "auto"), received "{0}"."#)]
    InvalidExportOptionValue(String),

    #[error(
        r#""{option_value}" was specified for "output.exports", but entry module "{}" has the following exports: {}"#,
        entry_module.display(),
        format_quoted_strings(exported_keys)
    )]
    IncompatibleExportOptionValue {
        option_value: String,
        exported_keys: Vec<String>,
        entry_module: PathBuf,
    },

    #[error(r#"Missing export "{binding}" has been shimmed in module "{}"."#, exporter.display())]
    ShimmedExport {
        binding: String,
        exporter: PathBuf,
    },

    #[error(r#""{export_name}" cannot be exported from "{}" as it is a reexport that references itself."#, exporter.display())]
    CircularReexport {
        exporter: PathBuf,
        export_name: String,
    },

    #[error(r#"Could not resolve "{specifier}" from "{}"."#, importer.display())]
    UnresolvedImport {
        specifier: String,
        importer: PathBuf,
    },

    #[error(r#"Invalid config: "{0}"."#)]
    InvalidConfig(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

fn format_paths(paths: &[PathBuf]) -> String {
    paths
        .iter()
        .map(|p| p.display().to_string())
        .collect::<Vec<_>>()
        .join(" -> ")
}

fn format_quoted_strings(list: &[impl AsRef<str>]) -> String {
    debug_assert!(!list.is_empty());
    let is_single_item = list.len() == 1;
    let mut quoted_list = list
        .iter()
        .map(|item| format!("\"{}\"", item.as_ref()))
        .collect::<Vec<_>>();
    if is_single_item {
        quoted_list[0].clone()
    } else {
        let last_item = quoted_list.pop().unwrap();
        format!("{} and {}", quoted_list.join(", "), last_item)
    }
}
