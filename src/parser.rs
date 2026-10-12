
use crate::{
    TokenKind,
    Token,
    Keyword,
    Expr,
    Stmt,
    BinaryOp
};

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    position: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Eof,
    InvalidToken(String),
    UnexpectedToken(String),
    TokenExpected(String),
    NotAKeyword(String),
    // UnsupportedKeyword(String),
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
    pub fn parse(&mut self) -> Result<Stmt> {
        let ast = self.parse_statement()?;
        
        if !self.is_at_end() {
            println!("Here? {:#?}", self.current()?);
            return Err(
                ParseError::UnexpectedToken(
                self.current()?.lexeme.to_string()
            ));
        }
        Ok(ast)
    }
}

/// Private parsing methods for keywords
impl<'a> Parser<'a> {
    fn parse_statement(&mut self) -> Result<Stmt> {
        match self.keyword()? {
            Keyword::Let => {
                self.parse_let_stmt()
            },
            Keyword::If => {
                self.parse_if_stmt()
            }
            _ => return Err(
                ParseError::NotAKeyword(
                    self.current()?.lexeme.to_string()
                )
            )
        }
    }

    fn parse_let_stmt(&mut self) -> Result<Stmt> {     
        self.advance()?;
        
        if self.current()?.kind != TokenKind::Identifier {
            return Err(
                ParseError::InvalidToken(
                    self.current()?.lexeme.to_string()
                )
            )
        }

        // Consume the identifier we confirmed
        let ident = self.advance()?.lexeme.to_string();

        if self.current()?.kind != TokenKind::Equals {
            println!("Second {:#?}", self.current()?);
            return Err(
                ParseError::InvalidToken(
                    self.current()?.lexeme.to_string()
                )
            )
        }

        // We confirmed it's an equals, so consume it
        self.advance()?;

        let expr = self.parse_expression()?;

        
        Ok(Stmt::Let {
            keyword: Keyword::Let,
            identifier: ident,
            expression: expr
        })
        
    }

    fn parse_if_stmt(&mut self) -> Result<Stmt> {
        self.advance()?;
        
        let condition = self.parse_expression()?;
        
        if self.current()?.kind != TokenKind::LBrace {
            let lbrace = "{";
            let current = self.current()?.lexeme.to_string();
            let msg = format!("Expected '{lbrace}' but found {current} instead");
            
            return Err(
                ParseError::InvalidToken(msg)
            )
        }

        // Consume the { we found
        self.advance()?;
        let mut body = vec![];

        while let Ok(current) = self.current() {
            if current.kind == TokenKind::RBrace {
                self.advance()?;
                
                break
            }
            
            body.push(self.parse_statement()?);

        }
        
        Ok(
            Stmt::If {
                keyword: Keyword::If,
                condition: condition,
                body: Box::new(body)
            }
        )
    }

}

/// Private parsing methods for expressions
impl<'a> Parser<'a> {
    /// Start the parsing by calling the operation parser with least precedence;
    /// here, we called addition (which also performs subtraction).
    /// 
    /// We grouped addition with subtraction and multiplication with division
    /// because both have equal precedence and we must perform them from left to right
    fn parse_expression(&mut self) -> Result<Expr> {
        self.parse_addition()
    }
    
    ///
    /// Since multiplication/division has higher precedence, we call parse_multiplication()
    /// to check if we have any multiplication/division operation, if there is any, it will
    /// set the return value in left expression
    /// 
    /// If we haven't reached the end of file yet; meaning we still get the current token,
    /// we will look for plus/minus operator and then consume it (it's just the increase in
    /// our current positino actually).
    /// And then we will call the parse_multiplication() to do the operation again and set it
    /// in the right operation by putting plus/minus operator in between.
    /// 
    /// The reason we dont have to check if there are still tokens left after consuming the operator
    /// is that; the parse_multiplication() already does that internally
    ///
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
        /*
         * Call parse_primary() to get the numbers/identifiers
         * or a binary (either containing numbers/identifiers or other binaries)
         */
        let mut left = self.parse_primary()?;

        while let Ok(token) = self.current() {
            let operator = match token.kind {
                TokenKind::Slash => BinaryOp::Divide,
                TokenKind::Star => BinaryOp::Multiply,
                _ => break,
            };

            /*
             * Consume the operator and call again to get the right numbers/identifiers
             * or a binary
             */
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

    /*
     * This is the actual fundamental function that identifies what we have and returns
     * numbers or identifiers or expressions by pattern matching
     * Others just indirectly call it and add respective operators; they dont do the underlying work
     */
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
                Ok(
                    Expr::Identifier(
                        self.advance()?.lexeme.to_string()
                    )
                )
            },
            /*
             * This is the start of other expression inclosed in () so we have to consume the (
             * and parse the expression again, which will not touch the closing parenthesis ),
             * so we have to consume the closing parenthesis ) if we encountered it, consume it 
             * and return the expression; indicating success.
             * 
             * If we didn't encounter the closing parenthesis, we return error saying we expected
             * a closing parenthesis
             */
            TokenKind::LParen => {
                self.advance()?;
                let expr = self.parse_expression()?; // Where recursion happens

                if self.is_at_end() {
                    return Err(
                        ParseError::TokenExpected(
                            "')' expected at the end".to_string()
                        )
                    )
                }
                
                let current = self.current()?;
                match current.kind {
                    TokenKind::RParen => {
                        self.advance()?;
                        return Ok(expr)
                    },
                    _ => {
                        println!("{:#?}", current);
                        return Err(
                            ParseError::TokenExpected(
                            ")".to_string()
                        ))
                    }
                }
            },
            _ => Err(
                ParseError::InvalidToken(
                token.lexeme.to_string()
            ))
        }
    }
}

/// All the helper methods for the parser
impl<'a> Parser<'a> {
    /// If there are some tokens left, we will most likely get an Ok(),
    /// from the get statement below
    fn current(&self) -> Result<&Token<'a>> {
        if let Some(current) = self.tokens.get(self.position) {
            // Return the token from the current position
            Ok(current)
        } else {
            // If we got None, maybe there are no tokens left, meaning we are
            // at Eof
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
        // Return the token from 1 position prior, which will be the one
        // which we just consumed, because we had just done +1
        self.tokens
            .get(self.position - 1)
            .ok_or(ParseError::Eof)
    }

    fn keyword(&self) -> Result<Keyword> {        
        match self.current()?.kind {
            TokenKind::Keyword(keyword) => {
                Ok(keyword)
            },
            _ => return Err(
                ParseError::NotAKeyword(
                    self.current()?.lexeme.to_string()
                )
            )
        }
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
