
mod parse;
mod language;
mod semantic;

pub use parse::*;
pub use language::*;
pub use semantic::*;

#[derive(Debug, Clone, PartialEq)]
pub enum RuppleFrontendError {
    Lex(LexError),
    Parse(ParseError),
    Type(TypeError),
}

/// Since the root package just wants to get the typed program from the frontend
/// and doesn't need to manually assemble Lexer, Parser, TypeChecker etc,
/// so expose a clean API to make it easier.
/// 
/// Tho the modules are still public just incase anything wants to use them directly
pub fn checked_program(source: &str) -> Result<TypedProgram, RuppleFrontendError> {
    let lexed = Lexer::new(source).lex()
        .map_err(|err| RuppleFrontendError::Lex(err))?;
    
    let program = Parser::new(lexed).parse()
        .map_err(|err| RuppleFrontendError::Parse(err))?;
    
    TypeChecker::new().check_program(program)
        .map_err(|err| RuppleFrontendError::Type(err))
}