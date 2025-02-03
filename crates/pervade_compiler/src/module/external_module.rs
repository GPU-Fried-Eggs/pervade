use std::collections::HashMap;

use oxc_index::IndexVec;

use crate::{ImportRecord, ImportRecordId, ModuleId, SymbolRef};

#[derive(Debug)]
pub struct ExternalModule {
    pub id: ModuleId,
    pub exec_order: u32,
    pub import_records: IndexVec<ImportRecordId, ImportRecord>,
    pub is_symbol_for_namespace_referenced: bool,
    pub symbols_imported_by_others: HashMap<Atom, SymbolRef>,
}
