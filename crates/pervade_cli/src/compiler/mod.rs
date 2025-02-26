mod builder;
mod program;
mod take_in;

use std::path::PathBuf;
use std::sync::Arc;

pub use builder::OxcAst;
use oxc::codegen::{CodeGenerator, CodegenOptions, CodegenReturn};
use oxc::span::SourceType;
pub use program::OxcProgram;
pub use take_in::TakeIn;

pub struct OxcCompiler;

impl OxcCompiler {
    pub fn parse(source: impl Into<Arc<str>>, ty: SourceType) -> OxcProgram {
        OxcProgram::new(source.into(), ty)
    }

    pub fn print(ast: &OxcProgram, filename: &str, enable_source_map: bool) -> CodegenReturn {
        CodeGenerator::new()
            .with_options(CodegenOptions {
                comments: true,
                source_map_path: enable_source_map.then(|| PathBuf::from(filename)),
                ..CodegenOptions::default()
            })
            .build(ast.program())
    }
}

#[cfg(test)]
mod tests {
    use oxc::allocator::Allocator;
    use oxc::ast::ast::Statement;
    use oxc::span::SPAN;

    use super::*;

    #[test]
    fn test_compiler() {
        let ast = OxcCompiler::parse("const a = 1;".to_string(), SourceType::default());
        let code = OxcCompiler::print(&ast, "", false).code;
        assert_eq!("const a = 1;\n", code);
    }

    #[test]
    fn test_ast_var_decl() {
        let allocator = Allocator::default();
        let snippet = OxcAst::new(&allocator);
        let var_decl = snippet.var_decl_stmt("d", snippet.number_expr(4.0, "4"));

        let mut ast = OxcProgram::new("".into(), SourceType::default());
        ast.program_mut().body.push(var_decl);

        let code = OxcCompiler::print(&ast, "", false).code;
        assert_eq!("var d = 4;\n", code);
    }

    #[test]
    fn test_ast_import_statement() {
        let allocator = Allocator::default();
        let snippet = OxcAst::new(&allocator);
        let import_stmt = snippet.import_star_stmt("Module", "Alias");

        let mut ast = OxcProgram::new("".into(), SourceType::default());
        ast.program_mut().body.push(import_stmt);

        let code = OxcCompiler::print(&ast, "", false).code;
        assert_eq!("import * as Alias from \"Module\";\n", code);
    }

    #[test]
    fn test_ast_function_call_with_arg() {
        let allocator = Allocator::default();
        let snippet = OxcAst::new(&allocator);
        let call_expr = snippet.call_expr_with_arg_expr(
            snippet.id_ref_expr("console.log", SPAN),
            snippet.string_literal_expr("Hello, World!", SPAN),
        );

        let mut ast = OxcProgram::new("".into(), SourceType::default());
        ast.program_mut().body.push(Statement::ExpressionStatement(
            snippet.builder.alloc_expression_statement(SPAN, call_expr),
        ));

        let code = OxcCompiler::print(&ast, "", false).code;
        assert_eq!("console.log(\"Hello, World!\");\n", code);
    }
}
