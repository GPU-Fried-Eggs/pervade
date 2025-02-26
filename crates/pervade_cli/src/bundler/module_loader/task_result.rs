use crate::bundler::symbols::SymbolMap;
use crate::bundler::module::NormalModuleBuilder;
use crate::bundler::resolve_id::ResolvedRequestInfo;
use crate::common::{ImportRecordId, ModuleId};
use crate::error::Error;

pub struct TaskResult {
    pub module_id: ModuleId,
    pub symbol_map: SymbolMap,
    pub resolved_deps: Vec<(ImportRecordId, ResolvedRequestInfo)>,
    pub errors: Vec<Error>,
    pub warnings: Vec<Error>,
    pub builder: NormalModuleBuilder,
}
