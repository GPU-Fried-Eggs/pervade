mod external_module;
mod normal_module;
mod normal_module_builder;

use std::collections::HashMap;

pub use external_module::ExternalModule;
pub use normal_module::{NormalModule, Resolution};
pub use normal_module_builder::NormalModuleBuilder;
use oxc::span::CompactStr;
use oxc_index::IndexVec;
use string_wizard::MagicString;

use crate::bundler::{symbols::Symbols, options::InputOptions};
use crate::common::{ImportRecord, ImportRecordId, ModuleId, SymbolRef};

#[derive(Debug)]
pub enum Module {
    Normal(Box<NormalModule>),
    External(Box<ExternalModule>),
}

impl Module {
    pub fn id(&self) -> ModuleId {
        match self {
            Module::Normal(v) => v.id,
            Module::External(v) => v.id,
        }
    }

    pub fn exec_order(&self) -> u32 {
        match self {
            Module::Normal(v) => v.exec_order,
            Module::External(v) => v.exec_order,
        }
    }

    pub fn import_records(&self) -> &IndexVec<ImportRecordId, ImportRecord> {
        match self {
            Module::Normal(m) => &m.import_records,
            Module::External(m) => &m.import_records,
        }
    }

    pub fn mark_symbol_for_namespace_referenced(&mut self) {
        match self {
            Module::Normal(m) => m.is_symbol_for_namespace_referenced = true,
            Module::External(m) => m.is_symbol_for_namespace_referenced = true,
        }
    }

    pub fn finalize(&mut self, ctx: ModuleFinalizeContext) {
        match self {
            Module::Normal(m) => m.finalize(ctx),
            Module::External(_) => unreachable!(),
        }
    }

    pub fn render(&self, ctx: ModuleRenderContext) -> Option<MagicString> {
        match self {
            Module::Normal(m) => m.render(ctx),
            Module::External(m) => m.render(ctx),
        }
    }
}

impl From<NormalModule> for Module {
    fn from(module: NormalModule) -> Self {
        Module::Normal(Box::new(module))
    }
}

impl From<ExternalModule> for Module {
    fn from(module: ExternalModule) -> Self {
        Module::External(Box::new(module))
    }
}

pub struct ModuleFinalizeContext<'a> {
    pub symbols: &'a Symbols,
    pub canonical_names: &'a HashMap<SymbolRef, CompactStr>,
}

pub struct ModuleRenderContext<'a> {
    pub symbols: &'a Symbols,
    pub final_names: &'a HashMap<SymbolRef, CompactStr>,
    pub input_options: &'a InputOptions,
}
