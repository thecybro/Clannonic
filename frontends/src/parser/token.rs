
use crate::language::Type;

/// The token kinds the language recognizes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TokenKind {
    /// foo
    Identifier, 

    /// 1, 2 etc
    IntegerLiteral,

    /// 1.0, 2.0, 3.5 etc
    FloatLiteral,

    /// "a_string", "smt"
    StringLiteral,

    /// true, false
    BooleanLiteral(BooleanVal), // This is for the boolean values; not boolean data type

    /// [`Keyword`]
    Keyword(Keyword),
    
    Plus,
    Minus,
    Star,
    Slash,

    PlusPlus,
    MinusMinus,

    PlusEquals,
    MinusEquals,
    
    Arrow, // -> Used to indicate function return type
            
    Equals, // =
    EqualsEquals, // ==

    GreaterThan, // >
    LessThan, // <
    Not, // !
    
    NotEquals, // !=
    
    GreaterOrEquals, // >=
    LessOrEquals, // <=

    AndAnd,
    OrOr,
    
    LParen, // (
    RParen, // )

    LBrace, // {
    RBrace, // }

    Colon, // :
    SemiColon, // ;
    Comma, // ,
    Dot, // .

    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BooleanVal {
    True,
    False
}

/// The keywords to create statements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Keyword {
    /// eg: `let x = 10`
    Let,

    /// let mut x = 10
    Mut, 
    For,

    /// ```no_run
    /// while true {
    ///     // body here
    /// }
    /// ```
    While,

    /// ```no_run
    /// fn add(a: int, b: int) -> int {
    ///     return a + b;
    /// }
    /// ```
    Fn, 

    /// ```no_run
    /// if true {
    ///     // body here
    /// }
    /// ```
    If, 
    Else,

    /// ```no_run
    /// fn add(a: int, b: int) -> int {
    ///     return a + b;
    /// }
    /// ```
    Return, 

    /// This is the variant for the reserved data type lexeme
    /// like int, String, bool, etc 
    Type(Type),
}

/// To store the location of the characters in the source code
/// and later to use it to display debug info
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}


/// What a token in this language has
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'a> {
    pub kind: TokenKind, // what our lexer gives
    pub lexeme: &'a str, // the actual text that we had lexed
    pub span: Span // how long is it? it's starting and ending position
}
