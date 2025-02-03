mod external_module;
mod normal_module;

use std::collections::HashMap;

use external_module::ExternalModule;
use normal_module::NormalModule;
use oxc::span::Atom;

use crate::{ModuleId, SymbolRef};

#[derive(Debug)]
pub enum Module {
    Normal(Box<NormalModule>),
    External(Box<ExternalModule>),
}

impl Module {
    pub fn id(&self) -> ModuleId {
        match self {
            Module::Normal(m) => m.id,
            Module::External(m) => m.id,
        }
    }

    pub fn exec_order(&self) -> u32 {
        match self {
            Module::Normal(v) => v.exec_order,
            Module::External(v) => v.exec_order,
        }
    }

    pub fn expect_normal(&self) -> &NormalModule {
        match self {
            Module::Normal(m) => m,
            Module::External(_) => unreachable!(),
        }
    }
}

pub struct ModuleFinalizeContext<'a> {
    pub canonical_names: &'a HashMap<SymbolRef, Atom>,
    pub symbols: &'a Symbols,
}
