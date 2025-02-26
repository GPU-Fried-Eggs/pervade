mod file_system;
mod file_system_memory;
mod file_system_os;

use std::path::{Path, PathBuf};
use std::sync::Arc;

pub use file_system::FileSystem;
pub use file_system_memory::MemoryFileSystem;
pub use file_system_os::OsFileSystem;
use oxc_resolver::{FsCache, ResolveOptions, ResolverGeneric};

use crate::error::Error;

#[derive(Debug)]
pub struct Resolver<T: FileSystem + Default = OsFileSystem> {
    cwd: PathBuf,
    inner: ResolverGeneric<FsCache<T>>,
}

#[derive(Debug)]
pub struct ResolveReturn {
    pub resolved: Arc<str>,
}

impl<F: FileSystem + Default> Resolver<F> {
    pub fn new(cwd: PathBuf, preserve_symlinks: bool, fs: F) -> Self {
        let resolve_options = ResolveOptions {
            symlinks: !preserve_symlinks,
            extensions: vec![".js", ".jsx", ".ts", ".tsx"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            prefer_relative: false,
            ..Default::default()
        };

        let inner_resolver =
            ResolverGeneric::new_with_cache(Arc::new(FsCache::new(fs)), resolve_options);
        Self {
            cwd,
            inner: inner_resolver,
        }
    }

    pub fn cwd(&self) -> &PathBuf {
        &self.cwd
    }

    pub fn resolve(&self, importer: Option<&Path>, specifier: &str) -> Result<ResolveReturn, Error> {
        let resolved = if let Some(importer) = importer {
            let context = importer.parent().expect("Should have a parent dir");
            self.inner.resolve(context, specifier)
        } else {
            let joined_specifier = self.cwd.join(specifier);

            let is_path_like = specifier.starts_with('.') || specifier.starts_with('/');

            let resolved = self
                .inner
                .resolve(&self.cwd, joined_specifier.to_str().unwrap());
            if resolved.is_ok() {
                resolved
            } else if !is_path_like {
                // If the specifier is not path-like, we should try to resolve it as a bare specifier. This allows us to resolve modules from node_modules.
                self.inner.resolve(&self.cwd, specifier)
            } else {
                resolved
            }
        };

        resolved
            .map(|info| ResolveReturn {
                resolved: info.path().to_string_lossy().into(),
            })
            .map_err(|_| {
                if let Some(importer) = importer {
                    Error::UnresolvedImport {
                        specifier: specifier.into(),
                        importer: importer.into(),
                    }
                } else {
                    Error::UnresolvedEntry {
                        unresolved_id: specifier.into(),
                    }
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    fn create_test_fs() -> MemoryFileSystem {
        let files = &[
            (
                &"/src/index.js".to_string(),
                &"console.log('hello');".to_string(),
            ),
            (
                &"/src/utils.ts".to_string(),
                &"export const utils = {};".to_string(),
            ),
            (
                &"/node_modules/lib/index.js".to_string(),
                &"export {};".to_string(),
            ),
        ];
        MemoryFileSystem::new(files)
    }

    #[test]
    fn test_relative_path_resolution() {
        let fs = create_test_fs();
        let resolver = Resolver::new(PathBuf::from("/src"), false, fs);

        let result = resolver.resolve(Some(Path::new("/src/index.js")), "./utils");

        assert!(result.is_ok());
        assert_eq!(
            Path::new("/src/utils.ts").to_string_lossy(),
            &result.unwrap().resolved[..],
        );
    }

    #[test]
    fn test_bare_specifier_resolution() {
        let fs = create_test_fs();
        let resolver = Resolver::new(PathBuf::from("/src"), false, fs);

        let result = resolver.resolve(None, "lib");

        assert!(result.is_ok());
        assert_eq!(
            Path::new("/node_modules/lib/index.js").to_string_lossy(),
            &result.unwrap().resolved[..],
        );
    }

    #[test]
    fn test_missing_module() {
        let fs = create_test_fs();
        let resolver = Resolver::new(PathBuf::from("/src"), false, fs);

        let result = resolver.resolve(Some(Path::new("/src/index.js")), "non-existent");

        assert!(result.is_err());
    }

    #[test]
    fn test_directory_resolution() {
        let mut fs = create_test_fs();
        fs.add_file(Path::new("/src/components/index.js"), "");

        let resolver = Resolver::new(PathBuf::from("/src"), false, fs);

        let result = resolver.resolve(Some(Path::new("/src/index.js")), "./components");

        assert!(result.is_ok());
        assert_eq!(
            Path::new("/src/components/index.js").to_string_lossy(),
            &result.unwrap().resolved[..],
        );
    }
}
