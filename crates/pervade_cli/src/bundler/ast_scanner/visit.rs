use oxc::ast::{ast, visit::walk, Visit};
use oxc_index::IndexVec;

use super::Scanner;

impl<'a> Visit<'a> for Scanner<'a> {
    fn visit_program(&mut self, program: &ast::Program<'a>) {
        self.result.stmt_infos = IndexVec::with_capacity(program.body.len());
        for (idx, stmt) in program.body.iter().enumerate() {
            self.current_stmt_info.stmt_idx = idx;
            self.visit_statement(stmt);
            self.result
                .stmt_infos
                .push(std::mem::take(&mut self.current_stmt_info));
        }
    }

    fn visit_binding_identifier(&mut self, ident: &ast::BindingIdentifier) {
        let symbol_id = ident.symbol_id.get().unwrap();
        if self.scope.root_scope_id() == self.symbol_table.get_scope_id(symbol_id) {
            self.add_declared_id(symbol_id)
        }
    }

    fn visit_statement(&mut self, stmt: &ast::Statement<'a>) {
        if let Some(decl) = stmt.as_module_declaration() {
            self.scan_module_decl(decl);
        }
        walk::walk_statement(self, stmt)
    }
}
