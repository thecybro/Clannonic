
mod lexer;
mod parser;

pub use lexer::*;
pub use parser::*;



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
    
    Arrow, // -> Used to indicate function return type
    
    DSlash, // //
    TSlash, // ///
    MSlash, // Any number of slashes above 3
        
    Equals, // =
    EqualsEquals, // ==

    GreaterThan, // >
    LessThan, // <
    Not, // !
    
    NotEquals, // !=
    
    GreaterOrEquals, // >=
    LessOrEquals, // <=

    LParen, // (
    RParen, // )

    LBrace, // {
    RBrace, // }

    Col, // :
    SemCol, // ;
    Comma, // ,
    Dot, // .

    Eof,
}


/// The data types this language currently supports
/// Usage example: 
/// ```no_run
/// fn main() -> int {
///     // do something here
/// }
/// ```
/// 
/// Here the return type is 'int' and
/// the function must indicate the type it is going to return
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DataType {
    Integer, // int
    Float, // float
    String, // String
    Boolean, // bool
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Boolean => writeln!(f, "bool"),
            DataType::Float => writeln!(f, "float"),
            DataType::String => writeln!(f, "string"),
            DataType::Integer => writeln!(f, "int")
        }
    }
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
    For,
    While, // Done
    
    Fn, // done (partially)
    If, // Done
    Else,
    Return, // done

    // This is the variant for the reserved data type lexeme
    // like int, String, bool, etc 
    DataType(DataType),
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


/// The operators you can use to compare stuff
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConditionalOp {
    IsEqualTo,
    IsNotEqualTo,
    IsGreaterThan,
    IsLessThan,
    IsGreaterOrEqualTo,
    IsLessOrEqualTo,
}


/// The binary operators for arithmetic operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum UnaryOp {
    Plus,
    Minus,
    Increment,
    Decrement,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryFixity {
    Prefix,
    Postfix,
}


/// The types of expressions supported
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Expr {
    Float(f64), 
    Integer(i64),
    Identifier(String),
    String(String),
    Boolean(BooleanVal), 
    Conditional {
        /// The reason we use Expr instead of Identifier
        /// because we might have to compare between
        /// any 2 expressions as well, like 
        /// ```no_run
        /// (a + b) >= (c + d)
        /// ```
        /// 
        /// here, the individual arithmetic operation expressions like
        /// (a + b) and (c + d) would ultimately be converted into an
        /// identifier or number but if we had made the left and right hand
        /// sides as not Expr, users wouldn't be able to write that
        left: Box<Expr>,
        operator: ConditionalOp,
        right: Box<Expr>
    },
    Binary {
        /// Obviously the reason we used Expr is because
        /// we need to have an AST which has the operations/
        /// expressions in their own respective nodes,
        /// 
        /// If we had made it 'left: TokenKind' instead of 
        /// `left: Box<Expr>`, we would be supporting only one 
        /// arithmetic operation chain.
        /// 
        /// For example:
        /// ```no_run
        /// // In the case of left/right: TokenKind
        /// let x = a + b // valid
        /// let x = a + b - c // invalid
        /// 
        /// // In the case of left/right: Box<Expr>
        /// let x = a + b // valid
        /// let x = a + b - c //valid
        /// 
        /// // And we can make this chain as long as we want,
        /// // like this is also perfectly valid:
        /// 
        /// let x = a + b - c * d / e
        /// 
        /// // Note tha the space between the expressions is not necessary,
        /// // like we can write this as well
        /// 
        /// let x = a+b-c*d/e // perfectly valid
        /// ```
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
        fixity: UnaryFixity,
    },
    /// A function call expression means using a function that has been implemented
    /// For example:
    /// ```no run
    /// 
    /// let x = main()
    /// ```
    FunctionCall {
        name: String,
        arguments: Box<[Expr]>,
    }
}

/// The supported statements and their structures
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Stmt {
    Let {
        identifier: String,
        expression: Expr
    },
    Assign {
      identifier: String,
      expression: Expr  
    },
    // I think the support for else if or elif branches is also
    // important here and they should have their own body instead
    // of the if statement having a single body
    If {
        condition: Expr,
        body: Box<[Stmt]>,
    },
    While {
        condition: Expr,
        body: Box<[Stmt]>,
    },
    /// A function statement means writing the implementation of a function
    /// For example:
    /// ```no_run
    /// 
    /// fn main() {
    ///     // implementation
    /// }
    /// ```
    Fn {
        name: String,
        parameters: Box<[FunctionParameter]>,
        return_type: Option<DataType>,
        return_expr: Option<Expr>,
        body: Box<[Stmt]>
    },
    Return(Expr),
    Expression(Expr),
}

#[derive(Debug,  Clone, PartialEq)]
pub struct FunctionParameter {
    pub name: String,
    pub data_type: DataType,
}

impl std::fmt::Display for FunctionParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.name)?;
        writeln!(f, " {}", self.data_type)
    }
}

#[derive(Debug, Clone)]
pub struct LexedOutput<'a> {
    pub tokens: Box<[Token<'a>]>
}


#[derive(Debug, Clone)]
pub struct Program {
    // This will allow us to return other stuff as part of the program
    // which are not the part of the ast itself
    pub ast: Box<[Stmt]>
}

