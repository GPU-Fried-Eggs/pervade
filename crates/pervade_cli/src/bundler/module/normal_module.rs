use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use oxc::semantic::{ReferenceId, ScopeTree, SymbolId};
use oxc::span::{Atom, CompactStr};
use oxc_index::IndexVec;
use string_wizard::MagicString;

use crate::bundler::symbols::Symbols;
use crate::bundler::module_finalizer::Finalizer;
use crate::common::{
    ImportRecord, ImportRecordId, LocalOrReExport, ModuleId, NamedImport, ResolvedExport,
    ResourceId, Specifier, StmtInfo, StmtInfoId, SymbolRef,
};
use crate::compiler::{OxcCompiler, OxcProgram};

use super::{Module, ModuleFinalizeContext, ModuleRenderContext};

#[derive(Debug)]
pub struct NormalModule {
    pub id: ModuleId,
    pub exec_order: u32,
    pub resource_id: ResourceId,
    pub ast: OxcProgram,
    pub named_imports: HashMap<SymbolId, NamedImport>,
    pub named_exports: HashMap<CompactStr, LocalOrReExport>,
    pub stmt_infos: IndexVec<StmtInfoId, StmtInfo>,
    pub import_records: IndexVec<ImportRecordId, ImportRecord>,
    pub star_exports: Vec<ImportRecordId>,

    pub resolved_exports: HashMap<CompactStr, ResolvedExport>,
    pub resolved_star_exports: Vec<ModuleId>,
    pub scope: ScopeTree,
    pub default_export_symbol: Option<SymbolId>,
    pub namespace_symbol: (SymbolRef, ReferenceId),
    pub is_symbol_for_namespace_referenced: bool,
}

pub enum Resolution {
    None,
    Ambiguous,
    Found(SymbolRef),
}

impl NormalModule {
    pub fn finalize(&mut self, ctx: ModuleFinalizeContext) {
        let (program, allocator) = self.ast.program_mut_and_allocator();
        let mut finalizer = Finalizer::new(FinalizeContext {
            allocator,
            symbols: ctx.symbols,
            scope: &self.scope,
            id: self.id,
            default_export_symbol: self.default_export_symbol,
            final_names: ctx.canonical_names,
        });
        finalizer.visit_program(program);
    }

    pub fn render(&self, _ctx: ModuleRenderContext) -> Option<MagicString> {
        let code = OxcCompiler::print(&self.ast, "", false).code;
        if code.is_empty() {
            None
        } else {
            let mut s = MagicString::new(code);
            s.prepend("// ");
            s.prepend(self.resource_id.as_str());
            s.prepend("\n");
            Some(s)
        }
    }

    // https://tc39.es/ecma262/#sec-getexportednames
    pub fn get_exported_names(
        &self,
        stack: &mut Vec<ModuleId>,
        modules: &IndexVec<ModuleId, Module>,
    ) -> HashSet<CompactStr> {
        if stack.contains(&self.id) {
            // cycle
            return Default::default();
        }

        stack.push(self.id);

        let ret: HashSet<CompactStr> = {
            self.star_exports
                .iter()
                .copied()
                .map(|id| &self.import_records[id])
                .flat_map(|rec| {
                    debug_assert!(rec.resolved_module.is_valid());
                    let importee = &modules[rec.resolved_module];
                    match importee {
                        Module::Normal(importee) => importee
                            .get_exported_names(stack, modules)
                            .into_iter()
                            .filter(|name| name.as_str() != "default"),
                        Module::External(_) => {
                            unimplemented!("handle external module")
                        }
                    }
                })
                .chain(self.named_exports.keys().map(|v| v.to_owned()))
                .collect()
        };

        stack.pop();

        ret
    }

    // https://tc39.es/ecma262/#sec-resolveexport
    pub fn resolve_export(
        &self,
        export_name: CompactStr,
        resolve_set: &mut Vec<(ModuleId, CompactStr)>,
        modules: &IndexVec<ModuleId, Module>,
        symbols: &mut Symbols,
    ) -> Resolution {
        let record = (self.id,  export_name.clone());
        if resolve_set.iter().rev().any(|prev| prev == &record) {
            unimplemented!("handle cycle")
        }
        resolve_set.push(record);

        let ret = if let Some(info) = self.named_exports.get(&export_name) {
            match info {
                LocalOrReExport::Local(local) => {
                    if let Some(named_import) = self.named_imports.get(&local.referenced.symbol) {
                        let record = &self.import_records[named_import.record_id];
                        let importee = &modules[record.resolved_module];
                        match importee {
                            Module::Normal(importee) => {
                                let resolved = match &named_import.imported {
                                    Specifier::Star => {
                                        Resolution::Found(importee.namespace_symbol.0)
                                    }
                                    Specifier::Literal(name) => {
                                        importee.resolve_export(name.to_owned(), resolve_set, modules, symbols)
                                    }
                                };
                                if let Resolution::Found(exist) = &resolved {
                                    symbols.union(local.referenced, *exist)
                                }
                                resolved
                            }
                            Module::External(_) => {
                                unimplemented!("handle external module")
                            }
                        }
                    } else {
                        Resolution::Found(local.referenced)
                    }
                }
                LocalOrReExport::Re(re) => {
                    let record = &self.import_records[re.record_id];
                    let importee = &modules[record.resolved_module];
                    match importee {
                        Module::Normal(importee) => match &re.imported {
                            Specifier::Star => Resolution::Found(importee.namespace_symbol.0),
                            Specifier::Literal(name) => {
                                importee.resolve_export(name.to_owned(), resolve_set, modules, symbols)
                            }
                        },
                        Module::External(_) => {
                            unimplemented!("handle external module")
                        }
                    }
                }
            }
        } else {
            if export_name.as_str() == "default" {
                return Resolution::None;
            }
            let mut star_resolution: Option<SymbolRef> = None;
            for e in &self.star_exports {
                let rec = &self.import_records[*e];
                let importee = &modules[rec.resolved_module];
                match importee {
                    Module::Normal(importee) => {
                        match importee.resolve_export(export_name.to_owned(), resolve_set, modules, symbols) {
                            Resolution::None => continue,
                            Resolution::Ambiguous => return Resolution::Ambiguous,
                            Resolution::Found(exist) => {
                                if let Some(star_resolution) = star_resolution {
                                    if star_resolution == exist {
                                        continue;
                                    } else {
                                        return Resolution::Ambiguous;
                                    }
                                } else {
                                    star_resolution = Some(exist)
                                }
                            }
                        }
                    }
                    Module::External(_) => {
                        unimplemented!("handle external module")
                    }
                }
            }

            star_resolution
                .map(Resolution::Found)
                .unwrap_or(Resolution::None)
        };

        resolve_set.pop();

        ret
    }

    pub fn resolve_star_exports(&self, modules: &IndexVec<ModuleId, Module>) -> Vec<ModuleId> {
        let mut visited = HashSet::new();
        let mut resolved = vec![];
        let mut queue = self
            .star_exports
            .iter()
            .map(|rec_id| {
                let rec = &self.import_records[*rec_id];
                rec.resolved_module
            })
            .collect::<Vec<_>>();

        while let Some(importee_id) = queue.pop() {
            if !visited.contains(&importee_id) {
                visited.insert(importee_id);
                resolved.push(importee_id);
                let importee = &modules[importee_id];
                match importee {
                    Module::Normal(importee) => queue.extend(
                        importee
                            .star_exports
                            .iter()
                            .map(|rec_id| importee.import_records[*rec_id].resolved_module),
                    ),
                    Module::External(_) => todo!(),
                }
            }
        }

        resolved
    }
}
