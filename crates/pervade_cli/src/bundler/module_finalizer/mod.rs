mod context;
mod visit_mut;

pub use context::FinalizeContext;

use crate::compiler::OxcAst;

pub struct Finalizer<'c, 'a> {
    pub ctx: FinalizeContext<'c>,
    pub snippet: OxcAst<'a>,
}

impl<'c, 'a> Finalizer<'c, 'a> {
    pub fn new(ctx: FinalizeContext<'a>) -> Self {
        Self { ctx }
    }
}
