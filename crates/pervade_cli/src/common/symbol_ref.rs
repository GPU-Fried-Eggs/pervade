use oxc::semantic::SymbolId;

use super::module_id::ModuleId;

/// Crossing module ref between symbols
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
