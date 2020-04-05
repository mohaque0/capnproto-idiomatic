mod ast;

use ast::Resolver;
use ast::Translator;
use ast::ToCode;

fn translate(cgr: &crate::parser::ast::CodeGeneratorRequest) -> ast::RustAst {
    let translated = ast::RustAst::translate(&ast::TranslationContext::new(), &cgr);

    let mut resolution_context = ast::ResolutionContext::new();
    ast::RustAst::build_context(&mut resolution_context, &translated);
    let resolved = ast::RustAst::resolve(
        &resolution_context,
        &translated
    );

    return resolved;
}

fn to_code(ast: &ast::RustAst) -> String {
    return ast.to_code();
}

pub fn code_gen(cgr: &crate::parser::ast::CodeGeneratorRequest) -> String {
    // Use this to view the cgr for debugging.
    //println!("{:#?}", cgr);
    let ast0 = translate(&cgr);
    let ast1 = ast::RustAst::generate_serde(
        &ast::SerdeGenerationContext::new(),
        &ast0
    );
    return to_code(&ast1);
}