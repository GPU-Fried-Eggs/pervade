mod bundle;
mod bundler;
mod compiler;
mod export;
mod graph;
mod import;
mod module;
mod module_loader;
mod options;

use std::{fmt::Debug, hash::Hash, path::Path, sync::Arc};

use oxc::{semantic::SymbolId, span::CompactStr};

pub use export::*;
pub use import::*;

oxc_index::define_index_type! {
    pub struct ModuleId = u32;
}
oxc_index::define_index_type! {
    pub struct ImportRecordId = u32;
}
oxc_index::define_index_type! {
    pub struct StmtInfoId = u32;
}

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct ImportRecordMeta: u8 {
        /// If it is `import * as ns from '...'` or `export * as ns from '...'`
        const CONTAINS_IMPORT_STAR = 1;
        /// If it is `import def from '...'`, `import { default as def }`, `export { default as def }` or `export { default } from '...'`
        const CONTAINS_IMPORT_DEFAULT = 1 << 1;
    }
}

#[derive(Debug)]
pub struct ImportRecord {
    /// `./lib.js` in `import { foo } from './lib.js';`
    pub module_request: CompactStr,
    pub resolved_module: ModuleId,
    pub meta: ImportRecordMeta,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SymbolRef {
    pub owner: ModuleId,
    pub symbol: SymbolId,
}

impl From<(ModuleId, SymbolId)> for SymbolRef {
    fn from(value: (ModuleId, SymbolId)) -> Self {
        Self {
            owner: value.0,
            symbol: value.1,
        }
    }
}

#[derive(Default, Debug)]
pub struct StmtInfo {
    pub stmt_id: usize,
    pub declared_symbols: Vec<SymbolId>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct ResourceId(Arc<str>);

impl ResourceId {
    pub fn new(value: impl Into<Arc<str>>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn stabilize(&self, cwd: &Path) -> String {
        stabilize_resource_id(&self.0, cwd)
    }
}

fn stabilize_resource_id(resource_id: &str, cwd: &Path) -> String {
    if resource_id.contains(':') || resource_id.contains('\0') {
        // handle virtual modules
        if resource_id.starts_with('\0') {
            return resource_id.replace('\0', "\\0");
        }
        return resource_id.to_string();
    }

    let path = Path::new(resource_id);

    if path.is_absolute() {
        if let Ok(relative_path) = path.strip_prefix(cwd) {
            return relative_path.to_string_lossy().into_owned();
        }
        return resource_id.to_string();
    }

    resource_id.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stabilize_resource_id() {
        let cwd = std::env::current_dir().unwrap();

        // absolute path
        let abs_path = cwd.join("src").join("main.js");
        assert_eq!(
            stabilize_resource_id(abs_path.to_str().unwrap(), &cwd),
            "src/main.js"
        );

        let abs_parent_path = cwd.join("..").join("src").join("main.js");
        assert_eq!(
            stabilize_resource_id(abs_parent_path.to_str().unwrap(), &cwd),
            "../src/main.js"
        );

        // non-path specifier
        assert_eq!(stabilize_resource_id("fs", &cwd), "fs");
        assert_eq!(
            stabilize_resource_id("https://deno.land/x/oak/mod.ts", &cwd),
            "https://deno.land/x/oak/mod.ts"
        );

        // virtual module
        assert_eq!(stabilize_resource_id("\0foo", &cwd), "\\0foo");
    }
}
