use std::collections::HashMap;

use oxc::allocator::Allocator;
use oxc::semantic::{ScopeTree, SymbolId};
use oxc::span::CompactStr;

use crate::bundler::symbols::Symbols;
use crate::common::{ModuleId, SymbolRef};

pub struct FinalizeContext<'ast> {
    pub id: ModuleId,
    pub allocator: &'ast Allocator,
    pub symbols: &'ast Symbols,
    pub scope: &'ast ScopeTree,
    pub final_names: &'ast HashMap<SymbolRef, CompactStr>,
    pub default_export_symbol: Option<SymbolId>,
}
