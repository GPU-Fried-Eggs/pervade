use std::collections::HashMap;

use oxc::semantic::{ReferenceId, SymbolId, SymbolTable};
use oxc::span::CompactStr;
use oxc_index::IndexVec;

use crate::common::{ModuleId, SymbolRef};

#[derive(Debug, Default)]
pub struct SymbolMap {
    pub names: IndexVec<SymbolId, CompactStr>,
    pub references: IndexVec<ReferenceId, Option<SymbolId>>,
}

impl SymbolMap {
    pub fn create_symbol(&mut self, name: CompactStr) -> SymbolId {
        self.names.push(name)
    }

    pub fn create_reference(&mut self, id: Option<SymbolId>) -> ReferenceId {
        self.references.push(id)
    }

    pub fn get_name(&self, id: SymbolId) -> &CompactStr {
        &self.names[id]
    }
}

impl From<SymbolTable> for SymbolMap {
    fn from(value: SymbolTable) -> Self {
        Self {
            names: value.names().map(|v| CompactStr::new(v)).collect(),
            references: value
                .references
                .iter()
                .map(|refer| refer.symbol_id())
                .collect(),
        }
    }
}

// Information about symbols for all modules
#[derive(Debug, Default)]
pub struct Symbols {
    pub(crate) tables: IndexVec<ModuleId, SymbolMap>,
    canonical_refs: IndexVec<ModuleId, HashMap<SymbolId, SymbolRef>>,
}

impl Symbols {
    pub fn new(tables: IndexVec<ModuleId, SymbolMap>) -> Self {
        Self {
            canonical_refs: tables.iter().map(|_table| HashMap::default()).collect(),
            tables,
        }
    }

    pub fn union(&mut self, a: SymbolRef, b: SymbolRef) {
        let root_a = self.get_canonical_ref(a);
        let root_b = self.get_canonical_ref(b);
        if root_a == root_b {
            return;
        }
        self.canonical_refs[a.owner].insert(a.symbol, root_b);
    }

    pub fn get_original_name(&self, refer: SymbolRef) -> &CompactStr {
        self.tables[refer.owner].get_name(refer.symbol)
    }

    pub fn get_canonical_ref(&mut self, target: SymbolRef) -> SymbolRef {
        let mut canonical = target;
        while let Some(founded) = self.canonical_refs[canonical.owner]
            .get(&canonical.symbol)
            .copied()
        {
            debug_assert!(founded != target);
            canonical = founded;
        }
        if target != canonical {
            self.canonical_refs[target.owner].insert(target.symbol, canonical);
        }
        canonical
    }

    pub fn par_get_canonical_ref(&self, target: SymbolRef) -> SymbolRef {
        let mut canonical = target;
        while let Some(founded) = self.canonical_refs[canonical.owner]
            .get(&canonical.symbol)
            .copied()
        {
            debug_assert!(founded != canonical);
            canonical = founded;
        }
        canonical
    }
}
