use oxc::span::CompactStr;

use super::module_id::ModuleId;

oxc_index::define_index_type! {
    pub struct ImportRecordId = u32;
}

#[derive(Debug)]
pub struct ImportRecord {
    /// `./lib.js` in `import { foo } from './lib.js';`
    pub module_request: CompactStr,
    pub resolved_module: ModuleId,
    // export * as ns from '...'
    // import * as ns from '...'
    pub is_import_namespace: bool,
}

impl ImportRecord {
    pub fn new(specifier: CompactStr) -> Self {
        Self {
            module_request: specifier,
            resolved_module: Default::default(),
            is_import_namespace: false,
        }
    }
}
