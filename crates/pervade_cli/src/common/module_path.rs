use std::fmt::Debug;
use std::hash::Hash;
use std::path::Path;
use std::sync::Arc;

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

impl AsRef<str> for ResourceId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

fn stabilize_resource_id(resource_id: &str, cwd: &Path) -> String {
    if resource_id.contains('\0') {
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
