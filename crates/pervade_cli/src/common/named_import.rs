use oxc::span::CompactStr;

use super::import_record::ImportRecordId;
use super::symbol_ref::SymbolRef;

/// This is a representation for statements like
/// - Case A: `import { foo } from 'foo'`
/// - Case B: `import * as fooNs from 'foo'`
/// - Case C: `import { foo as foo2 } from 'foo'`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedImport {
    /// For case A, the `imported` is `foo`.
    /// For case B, the `imported` is meaningless.
    /// For case C, the `imported` is `foo`.
    pub imported: Specifier,
    /// For case A, the `imported_as` is a `SymbolRef` from `foo`.
    /// For case B, the `imported_as` is a `SymbolRef` from `fooNs`.
    /// For case C, the `imported_as` is a `SymbolRef` from `foo2`.
    pub imported_as: SymbolRef,
    pub record_id: ImportRecordId,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Specifier {
    Star,
    Literal(CompactStr),
}

impl Specifier {
    pub fn is_star(&self) -> bool {
        matches!(self, Self::Star)
    }

    pub fn is_default(&self) -> bool {
        matches!(self, Self::Literal(atom) if atom.as_str() == "default")
    }
}

impl From<CompactStr> for Specifier {
    fn from(atom: CompactStr) -> Self {
        Self::Literal(atom)
    }
}
