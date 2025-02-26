use oxc::ast::{ast, VisitMut};

use super::Finalizer;

impl<'a> VisitMut<'a> for Finalizer<'a> {
    fn visit_program(&mut self, program: &mut ast::Program<'a>) {
        let alloc = self.ctx.allocator;
        program
            .body
            .retain(|stmt| self.should_keep_this_top_level_stmt(stmt));
        program.body.iter_mut().for_each(|stmt| match stmt {
            ast::Statement::ModuleDeclaration(decl) => match decl.0 {
                ast::ModuleDeclaration::ExportDefaultDeclaration(decl) => match &mut decl.declaration{
                    ast::ExportDefaultDeclarationKind::Expression(exp) => {
                        let mut declarations = Vec::new_in(self.ctx.allocator);
                        declarations.push(ast::VariableDeclarator {
                            span: Default::default(),
                            kind: ast::VariableDeclarationKind::Var,
                            id: ast::BindingPattern {
                                kind: ast::BindingPatternKind::BindingIdentifier(Box(
                                    self.ctx.allocator.alloc(BindingIdentifier {
                                        span: Default::default(),
                                        name: "".into(),
                                        symbol_id: Cell::new(self.ctx.default_export_symbol),
                                    }),
                                )),
                                type_annotation: None,
                                optional: false,
                            },
                            init: Some(exp.take_in(self.ctx.allocator)),
                            definite: false,
                        });
                        *stmt = ast::Statement::Declaration(Declaration::VariableDeclaration(Box(alloc
                            .alloc(VariableDeclaration {
                                span: Default::default(),
                                kind: ast::VariableDeclarationKind::Var,
                                declarations,
                                modifiers: Default::default(),
                            }))))
                    }
                    ast::ExportDefaultDeclarationKind::FunctionDeclaration(decl) => {
                        *stmt = ast::Statement::Declaration(
                            ast::Declaration::FunctionDeclaration(decl.take_in(alloc)),
                        )
                    }
                    ast::ExportDefaultDeclarationKind::ClassDeclaration(decl) => {
                        *stmt = ast::Statement::Declaration(
                            ast::Declaration::ClassDeclaration(decl.take_in(alloc)),
                        )
                    }
                    _ => {}
                },
                ast::ModuleDeclaration::ExportNamedDeclaration(named_decl) => {
                    if let Some(decl) = &mut named_decl.declaration {
                        *stmt = ast::Statement::Declaration(decl.take_in(alloc))
                    }
                }
                _ => {}
            },
            _ => {}
        });

        self.visit_statements(&mut program.body);
    }

    fn visit_identifier_reference(&mut self, ident: &mut ast::IdentifierReference) {
        if let Some(symbol_id) =
            self.ctx.symbols.tables[self.ctx.id].references[ident.reference_id.get().unwrap()]
        {
            let symbol_ref = (self.ctx.id, symbol_id).into();
            let final_ref = self.ctx.symbols.par_get_canonical_ref(symbol_ref);
            if let Some(name) = self.ctx.final_names.get(&final_ref) {
                if &ident.name != name {
                    ident.name = name.clone()
                }
            }
        }
    }

    fn visit_binding_identifier(&mut self, ident: &mut ast::BindingIdentifier) {
        if let Some(symbol_id) = ident.symbol_id.get() {
            let symbol_ref = (self.ctx.id, symbol_id).into();
            let final_ref = self.ctx.symbols.par_get_canonical_ref(symbol_ref);
            if let Some(name) = self.ctx.final_names.get(&final_ref) {
                if &ident.name != name {
                    ident.name = name.as_ref().into()
                }
            }
        }
    }
}
