use std::collections::HashMap;

use oxc::span::CompactStr;
use oxc_index::IndexVec;
use string_wizard::MagicString;

use crate::bundler::symbols::Symbols;
use crate::common::{ImportRecord, ImportRecordId, ModuleId, ResourceId, SymbolRef};

use super::ModuleRenderContext;

#[derive(Debug)]
pub struct ExternalModule {
    pub id: ModuleId,
    pub exec_order: u32,
    pub resource_id: ResourceId,
    pub import_records: IndexVec<ImportRecordId, ImportRecord>,
    pub is_symbol_for_namespace_referenced: bool,
    pub symbols_imported_by_others: HashMap<CompactStr, SymbolRef>,
}

impl ExternalModule {
    pub fn new(id: ModuleId, resource_id: ResourceId) -> Self {
        Self {
            id,
            exec_order: u32::MAX,
            resource_id,
            import_records: Default::default(),
            is_symbol_for_namespace_referenced: false,
            symbols_imported_by_others: Default::default(),
        }
    }

    pub fn render(&self, _ctx: ModuleRenderContext) -> Option<MagicString> {
        let mut rendered = MagicString::new(format!("import \"{}\"", self.resource_id.as_ref()));

        rendered.prepend(format!("// {}\n", self.resource_id.as_str()));
        rendered.append("\n");
        Some(rendered)
    }

    pub fn resolve_export(&mut self, symbols: &mut Symbols, exported: &CompactStr) -> SymbolRef {
        *self
            .symbols_imported_by_others
            .entry(exported.clone())
            .or_insert_with(|| {
                (
                    self.id,
                    symbols.tables[self.id].create_symbol(exported.clone()),
                )
                    .into()
            })
    }
}
