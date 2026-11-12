
use crate::language::Type;

/// The token kinds the language recognizes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TokenKind {
    Identifier, 

    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    BooleanLiteral(BooleanVal), // This is for the boolean values; not boolean data type

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
    Let, // Done
    Mut, // let mut x = 10
    For,
    While, // Done
    
    Fn, // done (partially)
    If, // Done
    Else,
    Return, // done

    // This is the variant for the reserved data type lexeme
    // like int, String, bool, etc 
    Type(Type),
}
