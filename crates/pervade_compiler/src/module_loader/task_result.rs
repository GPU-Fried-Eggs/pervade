use crate::{graph::SymbolMap, ImportRecordId, ModuleId};


pub struct TaskResult {
  pub module_id: ModuleId,
  pub symbol_map: SymbolMap,
  pub resolved_deps: Vec<(ImportRecordId, ResolvedRequestInfo)>,
  pub errors: Vec<BuildError>,
  pub warnings: Vec<BuildError>,
  pub builder: ModuleBuilder,
}
