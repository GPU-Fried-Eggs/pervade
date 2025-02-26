use std::path::Path;
use std::sync::Arc;

use crate::common::ResourceId;
use crate::error::Error;
use crate::resolver::Resolver;

pub struct ResolvedRequestInfo {
    pub path: Arc<str>,
    pub is_external: bool,
}

pub async fn resolve_id(
    resolver: &Resolver,
    request: &str,
    importer: Option<&ResourceId>,
    _preserve_symlinks: bool,
) -> Result<Option<ResolvedRequestInfo>, Error> {
    // external modules (non-entry modules that start with neither '.' or '/')
    // are skipped at this stage.
    if importer.is_some() && !request.starts_with('.') {
        Ok(None)
    } else {
        let resolved = resolver.resolve(Some(Path::new(importer.unwrap().as_str())), request)?;
        Ok(Some(ResolvedRequestInfo {
            path: resolved.resolved,
            is_external: false,
        }))
    }
}
