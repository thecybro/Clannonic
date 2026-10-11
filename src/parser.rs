
use crate::{
    TokenKind,
    Token,
    Expr,
    BinaryOp
};

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    position: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Eof,
    InvalidToken(TokenKind),
    UnexpectedToken(TokenKind),
    StringToI64,
}

type Result<T> = std::result::Result<T, ParseError>;

/// Public methods for the parser
impl<'a> Parser<'a> {
    /// The method that the caller needs to first call by passing a vector of
    /// tokens to later parse those tokens
    /// 
    /// Initialize the parser
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            tokens,
            position: 0
        }
    }

    /// Create an ast by calling the entry point parser function, which later
    /// leads to a recursive calling between itself, add operations parser,
    /// multiplication parser and primary parser which calls the add/multiplication
    /// parser or simply returns a number or identifier/variable by pattern matching
    pub fn parse(&mut self) -> Result<Expr> {
        let ast = self.parse_expression()?;
        
        if !self.is_at_end() {
            return Err(
                ParseError::UnexpectedToken(
                self.current()?.kind
            ));
        }
        Ok(ast)
    }
}

/// Private parsing methods
impl<'a> Parser<'a> {
    fn parse_expression(&mut self) -> Result<Expr> {
        self.parse_addition()
    }

    fn parse_addition(&mut self) -> Result<Expr> {
        let mut left = self.parse_multiplication()?;
        
        while let Ok(token) = self.current() {
            let operator = match token.kind {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Subtract,
                _ => break
            };

            self.advance()?;

            let right = self.parse_multiplication()?;

            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right)
            };
        }
        
        Ok(left)
    }

    fn parse_multiplication(&mut self) -> Result<Expr> {
        let mut left = self.parse_primary()?;

        while let Ok(token) = self.current() {
            let operator = match token.kind {
                TokenKind::Slash => BinaryOp::Divide,
                TokenKind::Star => BinaryOp::Multiply,
                _ => break,
            };

            self.advance()?;
            let right = self.parse_primary()?;

            left = Expr::Binary{
                left: Box::new(left),
                operator,
                right: Box::new(right)
            };
        }
        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expr> {
        let token = self.current()?;
        
        match token.kind {
            TokenKind::Integer => {
                if let Some(integer) = parse_to_i64(token.lexeme) {
                    self.advance()?;
                    Ok(Expr::Integer(integer))
                } else {
                    Err(ParseError::StringToI64)
                }
            },
            TokenKind::Identifier => {
                let identifier = self.advance()?;
                Ok(
                    Expr::Identifier(
                        identifier.lexeme.to_string()
                    )
                )
            },
            TokenKind::LParen => {
                self.advance()?;
                let expr = self.parse_expression()?; // Where recursion happens
                let current = self.current()?;
                match current.kind {
                    TokenKind::RParen => {
                        self.advance()?;
                        return Ok(expr)
                    },
                    _ => {
                        println!("{:#?}", current);
                        return Err(
                            ParseError::UnexpectedToken(
                            self.current()?.kind
                        ))
                    }
                }
            },
            _ => Err(
                ParseError::InvalidToken(
                token.kind
            ))
        }
    }
}

/// All the helper methods for the parser
impl<'a> Parser<'a> {
    fn current(&self) -> Result<&Token<'a>> {
        if let Some(current) = self.tokens.get(self.position) {
            Ok(current)
        } else {
            Err(ParseError::Eof)
        }
    }

    #[allow(unused)]
    fn peek(&self) -> Result<&Token<'a>> {
        if self.is_at_end() {
            return Err(ParseError::Eof)
        }

        if let Some(next) = self.tokens.get(self.position + 1) {
            Ok(next)
        } else {
            Err(ParseError::Eof)
        }
    }

    fn advance(&mut self) -> Result<&Token<'a>> {
        if self.is_at_end() {
            return Err(ParseError::Eof);
        }
    
        self.position += 1;
        self.tokens
            .get(self.position - 1)
            .ok_or(ParseError::Eof)
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }
}

/// An external helper for parsing strings to i64
fn parse_to_i64(input: &str) -> Option<i64> {
    if let Ok(integer) = input.parse::<i64>() {
        Some(integer)
    } else {
        None
    }
}
