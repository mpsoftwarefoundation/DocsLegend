mod errors;
mod generation;
mod lexing;
mod parsing;
mod syntax;
use crate::{
    errors::standard_error::StandardError, generation::generator::Generator, lexing::lexer::Lexer,
    parsing::parser::Parser,
};

pub fn generate_site(filename: &str, output_dir: &str, code: &str) -> Result<(), StandardError> {
    let mut lexer = Lexer::new(filename, code.to_string());
    let tokens = match lexer.make_tokens() {
        Ok(tok) => tok,
        Err(e) => return Err(e),
    };

    let mut parser = Parser::new(&tokens);
    let parsed = parser.parse();

    if let Some(err) = parsed.error {
        return Err(err);
    }

    let generator = Generator::new(output_dir);
    let _ = generator.visit(parsed.page.unwrap());

    Ok(())
}
