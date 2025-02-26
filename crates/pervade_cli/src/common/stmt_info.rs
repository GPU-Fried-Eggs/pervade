use oxc::semantic::SymbolId;

oxc_index::define_index_type! {
    pub struct StmtInfoId = u32;
}

#[derive(Default, Debug)]
pub struct StmtInfo {
    pub stmt_idx: usize,
    // currently, we only store top level symbols
    pub declared_symbols: Vec<SymbolId>,
}
