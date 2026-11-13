
use crate::language::{
    BinaryOp,
    ConditionalOp,
    UnaryFixity,
    UnaryOp,
    AssignOp,
    Type,
};
use super::BooleanVal;

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
    /// 
    /// Here, the expression is the main() part, whereas the whole line is a let statement
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
        mutable: bool,
        annotation: Option<Type>,
        expression: Expr
    },
    Assign {
      identifier: String,
      operator: AssignOp,
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
    Function {
        name: String,
        parameters: Box<[FunctionParameter]>,
        return_type: Type,
        body: Box<[Stmt]>
    },
    Return(Option<Expr>),
    Expression(Expr),
}

#[derive(Debug,  Clone, PartialEq)]
pub struct FunctionParameter {
    pub name: String,
    pub data_type: Type,
}

impl std::fmt::Display for FunctionParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.name)?;
        writeln!(f, " {:#?}", self.data_type)
    }
}
