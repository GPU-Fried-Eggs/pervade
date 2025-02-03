use oxc::{semantic::ReferenceId, span::CompactStr};

use crate::{ImportRecordId, SymbolRef};

#[derive(Debug)]
pub struct LocalExport {
    pub referenced: SymbolRef,
}

impl From<LocalExport> for LocalOrReExport {
    fn from(value: LocalExport) -> Self {
        Self::Local(value)
    }
}

#[derive(Debug)]
pub struct ReExport {
    pub imported: CompactStr,
    pub is_imported_star: bool,
    pub record_id: ImportRecordId,
}

impl From<ReExport> for LocalOrReExport {
    fn from(value: ReExport) -> Self {
        Self::Re(value)
    }
}

#[derive(Debug)]
pub enum LocalOrReExport {
    Local(LocalExport),
    Re(ReExport),
}

#[derive(Debug, Clone)]
pub struct ResolvedExport {
    pub local_symbol: SymbolRef,
    pub local_ref: ReferenceId,
}
