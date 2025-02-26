use oxc::allocator::{self, Allocator, Box, IntoIn};
use oxc::ast::{
    ast::{self, Argument, Expression, FunctionType, ImportOrExportKind, PropertyKind, Statement},
    AstBuilder, NONE,
};
use oxc::span::{Atom, Span, SPAN};

use super::take_in::TakeIn;

type PassedStr<'a> = &'a str;

pub struct OxcAst<'a> {
    pub builder: AstBuilder<'a>,
}

impl<'a> OxcAst<'a> {
    pub fn new(alloc: &'a Allocator) -> Self {
        Self {
            builder: AstBuilder::new(alloc),
        }
    }

    #[inline]
    pub fn alloc(&self) -> &'a Allocator {
        self.builder.allocator
    }

    pub fn atom(&self, value: &str) -> Atom<'a> {
        self.builder.atom(value)
    }

    #[inline]
    pub fn id(&self, name: PassedStr, span: Span) -> ast::BindingIdentifier<'a> {
        self.builder.binding_identifier(span, name)
    }

    #[inline]
    pub fn alloc_id_ref(
        &self,
        name: PassedStr,
        span: Span,
    ) -> Box<'a, ast::IdentifierReference<'a>> {
        self.builder.alloc_identifier_reference(span, name)
    }

    #[inline]
    pub fn id_name(&self, name: PassedStr, span: Span) -> ast::IdentifierName<'a> {
        self.builder.identifier_name(span, name)
    }

    #[inline]
    pub fn id_ref_expr(&self, name: PassedStr, span: Span) -> ast::Expression<'a> {
        self.builder.expression_identifier_reference(span, name)
    }

    /// `[object].[property]`
    pub fn literal_prop_access_member_expr(
        &self,
        object: PassedStr,
        property: PassedStr,
    ) -> ast::MemberExpression<'a> {
        ast::MemberExpression::StaticMemberExpression(self.builder.alloc_static_member_expression(
            SPAN,
            self.id_ref_expr(object, SPAN),
            self.builder.identifier_name(SPAN, property),
            false,
        ))
    }

    /// `[object].[property]`
    #[inline]
    pub fn literal_prop_access_member_expr_expr(
        &self,
        object: PassedStr,
        property: PassedStr,
    ) -> ast::Expression<'a> {
        ast::Expression::from(self.literal_prop_access_member_expr(object, property))
    }

    /// `name()`
    #[inline]
    pub fn call_expr(&self, name: PassedStr) -> ast::CallExpression<'a> {
        self.builder.call_expression(
            SPAN,
            self.builder.expression_identifier_reference(SPAN, name),
            NONE,
            self.builder.vec(),
            false,
        )
    }

    /// `name()`
    pub fn call_expr_expr(&self, name: PassedStr) -> ast::Expression<'a> {
        self.builder.expression_call(
            SPAN,
            self.builder.expression_identifier_reference(SPAN, name),
            NONE,
            self.builder.vec(),
            false,
        )
    }

    /// `name(arg)`
    pub fn call_expr_with_arg_expr(
        &self,
        name: ast::Expression<'a>,
        arg: ast::Expression<'a>,
    ) -> ast::Expression<'a> {
        let mut call_expr = self.simple_call_expr(name);
        call_expr.arguments.push(arg.into());
        ast::Expression::CallExpression(call_expr.into_in(self.alloc()))
    }

    /// `name(arg)`
    pub fn call_expr_with_arg_expr_expr(
        &self,
        name: PassedStr,
        arg: ast::Expression<'a>,
    ) -> ast::Expression<'a> {
        let arg = ast::Argument::from(arg);
        let mut call_expr = self.call_expr(name);
        call_expr.arguments.push(arg);
        ast::Expression::CallExpression(call_expr.into_in(self.alloc()))
    }

    /// `name(arg1, arg2)`
    pub fn call_expr_with_2arg_expr(
        &self,
        name: ast::Expression<'a>,
        arg1: ast::Expression<'a>,
        arg2: ast::Expression<'a>,
    ) -> ast::Expression<'a> {
        let mut call_expr =
            self.builder
                .call_expression(SPAN, name, NONE, self.builder.vec(), false);
        call_expr.arguments.push(arg1.into());
        call_expr.arguments.push(arg2.into());
        ast::Expression::CallExpression(call_expr.into_in(self.alloc()))
    }

    /// `name(arg1, arg2)`
    pub fn alloc_call_expr_with_2arg_expr_expr(
        &self,
        name: PassedStr,
        arg1: ast::Expression<'a>,
        arg2: ast::Expression<'a>,
    ) -> ast::Expression<'a> {
        self.builder.expression_call(
            SPAN,
            self.builder.expression_identifier_reference(SPAN, name),
            NONE,
            self.builder
                .vec_from_iter([Argument::from(arg1), Argument::from(arg2)]),
            false,
        )
    }

    /// `name(arg1, arg2)`
    pub fn call_expr_with_2arg_expr_expr(
        &self,
        name: ast::Expression<'a>,
        arg1: ast::Expression<'a>,
        arg2: ast::Expression<'a>,
    ) -> ast::Expression<'a> {
        self.builder.expression_call(
            SPAN,
            name,
            NONE,
            self.builder
                .vec_from_iter([Argument::from(arg1), Argument::from(arg2)]),
            false,
        )
    }

    /// `name()`
    #[inline]
    pub fn call_expr_stmt(&self, name: PassedStr) -> ast::Statement<'a> {
        self.builder
            .statement_expression(SPAN, self.call_expr_expr(name))
    }

    /// `var [name] = [init]`
    #[inline]
    pub fn var_decl_stmt(
        &self,
        name: PassedStr,
        init: ast::Expression<'a>,
    ) -> ast::Statement<'a> {
        ast::Statement::from(self.decl_var_decl(name, init))
    }

    /// `var [name] = [init]`
    pub fn decl_var_decl(
        &self,
        name: PassedStr,
        init: ast::Expression<'a>,
    ) -> ast::Declaration<'a> {
        let declarations = self.builder.vec1(
            self.builder.variable_declarator(
                SPAN,
                ast::VariableDeclarationKind::Var,
                self.builder.binding_pattern(
                    self.builder
                        .binding_pattern_kind_binding_identifier(SPAN, name),
                    NONE,
                    false,
                ),
                Some(init),
                false,
            ),
        );

        ast::Declaration::VariableDeclaration(self.builder.alloc_variable_declaration(
            SPAN,
            ast::VariableDeclarationKind::Var,
            declarations,
            false,
        ))
    }

    /// `var [name] = [init]`
    pub fn var_decl(
        &self,
        name: PassedStr,
        init: ast::Expression<'a>,
    ) -> Box<'a, ast::VariableDeclaration<'a>> {
        let declarations = self.builder.vec1(
            self.builder.variable_declarator(
                SPAN,
                ast::VariableDeclarationKind::Var,
                self.builder.binding_pattern(
                    self.builder
                        .binding_pattern_kind_binding_identifier(SPAN, name),
                    NONE,
                    false,
                ),
                Some(init),
                false,
            ),
        );
        self.builder.alloc_variable_declaration(
            SPAN,
            ast::VariableDeclarationKind::Var,
            declarations,
            false,
        )
    }

    /// `var [name];`
    pub fn var_decl_without_init(
        &self,
        name: PassedStr,
    ) -> Box<'a, ast::VariableDeclaration<'a>> {
        let declarations = self.builder.vec1(
            self.builder.variable_declarator(
                SPAN,
                ast::VariableDeclarationKind::Var,
                self.builder.binding_pattern(
                    self.builder
                        .binding_pattern_kind_binding_identifier(SPAN, name),
                    NONE,
                    false,
                ),
                None,
                false,
            ),
        );
        self.builder.alloc_variable_declaration(
            SPAN,
            ast::VariableDeclarationKind::Var,
            declarations,
            false,
        )
    }

    pub fn var_decl_multiple_names(
        &self,
        names: &[(&str, &str)],
        init: ast::Expression<'a>,
    ) -> Box<'a, ast::VariableDeclaration<'a>> {
        let mut declarations = self.builder.vec_with_capacity(1);
        let mut properties = self.builder.vec();
        names.iter().for_each(|(imported, local)| {
            properties.push(
                self.builder.binding_property(
                    SPAN,
                    self.builder.property_key_identifier_name(SPAN, *imported),
                    self.builder.binding_pattern(
                        self.builder
                            .binding_pattern_kind_binding_identifier(SPAN, *local),
                        NONE,
                        false,
                    ),
                    false,
                    false,
                ),
            );
        });
        declarations.push(ast::VariableDeclarator {
            id: ast::BindingPattern {
                kind: ast::BindingPatternKind::ObjectPattern(
                    ast::ObjectPattern {
                        properties,
                        ..TakeIn::dummy(self.alloc())
                    }
                    .into_in(self.alloc()),
                ),
                ..TakeIn::dummy(self.alloc())
            },
            init: Some(init),
            ..TakeIn::dummy(self.alloc())
        });
        self.builder.alloc_variable_declaration(
            SPAN,
            ast::VariableDeclarationKind::Var,
            declarations,
            false,
        )
    }

    /// ```js
    ///  var require_foo = __commonJS((exports, module) => {
    ///    ...
    ///  });
    /// ```
    pub fn commonjs_wrapper_stmt(
        &self,
        binding_name: PassedStr,
        commonjs_expr: ast::Expression<'a>,
        statements: allocator::Vec<'a, Statement<'a>>,
    ) -> ast::Statement<'a> {
        // (exports, module) => {}

        let mut params = self.builder.formal_parameters(
            SPAN,
            ast::FormalParameterKind::Signature,
            self.builder.vec_with_capacity(1),
            NONE,
        );
        let body = self
            .builder
            .function_body(SPAN, self.builder.vec(), statements);

        params.items.push(
            self.builder.formal_parameter(
                SPAN,
                self.builder.vec(),
                self.builder.binding_pattern(
                    self.builder
                        .binding_pattern_kind_binding_identifier(SPAN, "exports"),
                    NONE,
                    false,
                ),
                None,
                false,
                false,
            ),
        );

        params.items.push(
            self.builder.formal_parameter(
                SPAN,
                self.builder.vec(),
                self.builder.binding_pattern(
                    self.builder
                        .binding_pattern_kind_binding_identifier(SPAN, "module"),
                    NONE,
                    false,
                ),
                None,
                false,
                false,
            ),
        );

        //  __commonJS(...)
        let mut commonjs_call_expr =
            self.builder
                .call_expression(SPAN, commonjs_expr, NONE, self.builder.vec(), false);

        let arrow_expr = self
            .builder
            .alloc_arrow_function_expression(SPAN, false, false, NONE, params, NONE, body);
        commonjs_call_expr
            .arguments
            .push(ast::Argument::ArrowFunctionExpression(arrow_expr));

        // var require_foo = ...
        let var_decl_stmt = self.var_decl_stmt(
            binding_name,
            ast::Expression::CallExpression(commonjs_call_expr.into_in(self.alloc())),
        );

        var_decl_stmt
    }

    /// ```js
    /// var init_foo = __esm(() => { ... });
    /// ```
    pub fn esm_wrapper_stmt(
        &self,
        binding_name: PassedStr,
        esm_fn_expr: ast::Expression<'a>,
        statements: allocator::Vec<'a, Statement<'a>>,
        profiler_names: bool,
        stable_id: &str,
    ) -> ast::Statement<'a> {
        // () => { ... }
        let params = self.builder.formal_parameters(
            SPAN,
            ast::FormalParameterKind::Signature,
            self.builder.vec(),
            NONE,
        );
        let body = self
            .builder
            .function_body(SPAN, self.builder.vec(), statements);

        //  __esm(...)
        let mut esm_call_expr =
            self.builder
                .call_expression(SPAN, esm_fn_expr, NONE, self.builder.vec(), false);

        if profiler_names {
            let obj_expr = self.builder.alloc_object_expression(
                SPAN,
                self.builder.vec1(
                    self.builder.object_property_kind_object_property(
                        SPAN,
                        PropertyKind::Init,
                        ast::PropertyKey::from(
                            self.builder
                                .expression_string_literal(SPAN, stable_id, None),
                        ),
                        self.builder.expression_function(
                            SPAN,
                            FunctionType::FunctionExpression,
                            None,
                            false,
                            false,
                            false,
                            NONE,
                            NONE,
                            params,
                            NONE,
                            Some(body),
                        ),
                        true,
                        false,
                        false,
                    ),
                ),
                None,
            );
            esm_call_expr
                .arguments
                .push(ast::Argument::ObjectExpression(obj_expr));
        } else {
            let arrow_expr = self
                .builder
                .alloc_arrow_function_expression(SPAN, false, false, NONE, params, NONE, body);
            esm_call_expr
                .arguments
                .push(ast::Argument::ArrowFunctionExpression(arrow_expr));
        };

        // var init_foo = ...

        self.var_decl_stmt(
            binding_name,
            ast::Expression::CallExpression(esm_call_expr.into_in(self.alloc())),
        )
    }

    /// ```js
    /// (a, b)
    /// ```
    pub fn seq2_in_paren_expr(
        &self,
        a: ast::Expression<'a>,
        b: ast::Expression<'a>,
    ) -> ast::Expression<'a> {
        let mut expressions = self.builder.vec_with_capacity(2);
        expressions.push(a);
        expressions.push(b);
        let seq_expr = ast::Expression::SequenceExpression(
            self.builder.alloc_sequence_expression(SPAN, expressions),
        );
        ast::Expression::ParenthesizedExpression(
            self.builder.alloc_parenthesized_expression(SPAN, seq_expr),
        )
    }

    pub fn number_expr(&self, value: f64, raw: &'a str) -> ast::Expression<'a> {
        ast::Expression::NumericLiteral(self.builder.alloc_numeric_literal(
            SPAN,
            value,
            Some(Atom::from(raw)),
            oxc::syntax::number::NumberBase::Decimal,
        ))
    }

    /// ```js
    ///  id = ...
    /// ￣￣ AssignmentTarget
    /// ```
    pub fn simple_id_assignment_target(
        &self,
        id: PassedStr,
        span: Span,
    ) -> ast::AssignmentTarget<'a> {
        ast::AssignmentTarget::AssignmentTargetIdentifier(self.alloc_id_ref(id, span))
    }

    /// ```js
    /// () => xx
    /// ```
    pub fn only_return_arrow_expr(&self, expr: ast::Expression<'a>) -> ast::Expression<'a> {
        let statements = self.builder.vec1(ast::Statement::ExpressionStatement(
            self.builder.alloc_expression_statement(SPAN, expr),
        ));
        ast::Expression::ArrowFunctionExpression(
            self.builder.alloc_arrow_function_expression(
                SPAN,
                true,
                false,
                NONE,
                self.builder.formal_parameters(
                    SPAN,
                    ast::FormalParameterKind::Signature,
                    self.builder.vec(),
                    NONE,
                ),
                NONE,
                self.builder
                    .function_body(SPAN, self.builder.vec(), statements),
            ),
        )
    }

    /// `undefined` is acting like identifier, it might be shadowed by user code.
    #[inline]
    pub fn void_zero(&self) -> ast::Expression<'a> {
        self.builder.void_0(SPAN)
    }

    pub fn alloc_string_literal(
        &self,
        value: PassedStr,
        span: Span,
    ) -> Box<'a, ast::StringLiteral<'a>> {
        self.builder.alloc_string_literal(span, value, None)
    }

    pub fn string_literal_expr(&self, value: PassedStr, span: Span) -> ast::Expression<'a> {
        ast::Expression::StringLiteral(self.alloc_string_literal(value, span))
    }

    pub fn import_star_stmt(&self, source: PassedStr, as_name: PassedStr) -> ast::Statement<'a> {
        let specifiers =
            self.builder
                .vec1(ast::ImportDeclarationSpecifier::ImportNamespaceSpecifier(
                    self.builder
                        .alloc_import_namespace_specifier(SPAN, self.id(as_name, SPAN)),
                ));
        ast::Statement::ImportDeclaration(self.builder.alloc_import_declaration(
            SPAN,
            Some(specifiers),
            self.builder.string_literal(SPAN, source, None),
            None,
            NONE,
            ImportOrExportKind::Value,
        ))
    }

    pub fn alloc_simple_call_expr(
        &self,
        callee: Expression<'a>,
    ) -> allocator::Box<'a, ast::CallExpression<'a>> {
        self.builder
            .alloc_call_expression(SPAN, callee, NONE, self.builder.vec(), false)
    }

    pub fn simple_call_expr(&self, callee: Expression<'a>) -> ast::CallExpression<'a> {
        self.builder
            .call_expression(SPAN, callee, NONE, self.builder.vec(), false)
    }
}
