use std::sync::Arc;
use std::{fmt::Debug, pin::Pin};

use oxc::allocator::Allocator;
use oxc::ast::ast::Program;
use oxc::parser::Parser;
use oxc::semantic::{Semantic, SemanticBuilder};
use oxc::span::SourceType;

pub struct OxcProgram {
    program: Program<'static>,
    source: Pin<Arc<str>>,
    allocator: Pin<Box<Allocator>>,
}

impl OxcProgram {
    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn program(&self) -> &Program<'_> {
        unsafe { std::mem::transmute(&self.program) }
    }

    pub fn program_mut(&mut self) -> &mut Program<'_> {
        unsafe { std::mem::transmute(&mut self.program) }
    }

    pub fn program_mut_and_allocator(&mut self) -> (&mut Program<'_>, &Allocator) {
        let program = unsafe { std::mem::transmute(&mut self.program) };
        (program, &self.allocator)
    }

    pub fn make_semantic(&self) -> Semantic<'_> {
        let semantic = SemanticBuilder::new().build(self.program()).semantic;
        unsafe { std::mem::transmute(semantic) }
    }
}

unsafe impl Send for OxcProgram {}
unsafe impl Sync for OxcProgram {}

impl Debug for OxcProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ast").field("source", &self.source).finish()
    }
}

pub struct OxcCompiler;

impl OxcCompiler {
    pub fn parse(source: impl Into<Arc<str>>, ty: SourceType) -> OxcProgram {
        let source = Pin::new(source.into());
        let allocator = Box::pin(oxc::allocator::Allocator::default());
        let program = unsafe {
            let source = std::mem::transmute::<_, &'static str>(&*source);
            let alloc = std::mem::transmute::<_, &'static Allocator>(allocator.as_ref());
            Parser::new(alloc, source, ty).parse().program
        };

        OxcProgram {
            program,
            source,
            allocator,
        }
    }

    pub fn print() {}
}
