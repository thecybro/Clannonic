
use crate::{
    TokenKind,
    Token,
    Keyword,
    Expr,
    Stmt,
    BinaryOp,
    ConditionalOp
};

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    position: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Eof,
    InvalidToken(String),
    // UnexpectedToken(String),
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
    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut ast = vec![];

        // Loop over the tokens and parse the statements until we reach
        // the end of the tokens
        loop {
            if self.is_at_end() {
                break
            }

            ast.push(self.parse_statement()?)
        }

        // If somehow we didn't reach the end (which is unlikely cuz
        // the loop above ensures it runs until we reach the end)
        // But still, we aren't losing anything by keeping this, 
        // are we?
        if !self.is_at_end() {
            return Err(
                ParseError::InvalidToken(
                    self.current()?.lexeme.to_string()
                )
            )
        }
            
        Ok(ast)
    }
}

/// Private parsing methods for statements
impl<'a> Parser<'a> {
    /// Look at the tokens we have just gotten and see if we have any keywords
    /// in it, if we found any, call the respective parser to parse that keyword's 
    /// section.
    /// 
    /// Note that it returns after parsing the code/tokens associated with that statement
    /// like for example in the following code, if our position is 0, it will parse the
    /// let statement and then return, it won't parse the if statement
    /// 
    /// ```
    /// let x = a + b
    /// if x > 10 {
    ///     // do something
    /// }
    /// ```
    fn parse_statement(&mut self) -> Result<Stmt> {   
        // see what keyword we have here and call the appropriate
        // parser
        match self.keyword()? {
            Keyword::Let => {
                self.parse_let_stmt()
            },
            Keyword::If => {
                self.parse_if_stmt()
            },
            Keyword::While => {
                self.parse_while_stmt()
            },
            _ => return Err(
                ParseError::NotAKeyword(
                    self.current()?.lexeme.to_string()
                )
            )
        }
    }

    /// Parse the statement if it starts with let
    fn parse_let_stmt(&mut self) -> Result<Stmt> {  
        self.advance()?; // consume the let token

        // We gotta make sure that after let, we have an identifier
        // like 'let x'
        if self.current()?.kind != TokenKind::Identifier {
            return Err(
                ParseError::InvalidToken(
                    self.current()?.lexeme.to_string()
                )
            )
        }

        // Consume the identifier we confirmed
        let identifier = self.advance()?.lexeme.to_string();

        // After identifier, we need to have equals token
        if self.current()?.kind != TokenKind::Equals {
            return Err(
                ParseError::InvalidToken(
                    self.current()?.lexeme.to_string()
                )
            )
        }

        // We confirmed it's an equals, so consume it
        self.advance()?;

        // Let statement contains expression like 'a+b' after = sign,
        // we parse that expression
        let expression = self.parse_expression()?;

        Ok(Stmt::Let {
            keyword: Keyword::Let,
            identifier,
            expression,
        })
        
    }

    /// Parse the section of the tokens that come under the if statement
    fn parse_if_stmt(&mut self) -> Result<Stmt> {
        self.advance()?; // consume the if token

        // We would have an expression after if, so start the expression 
        // parsing, which will call either binary parsers or conditional
        // parsers based on what the expression is like
        let condition = self.parse_expression()?;

        // We gotta make sure we have a { after the expression 
        if self.current()?.kind != TokenKind::LBrace {
            return Err(
                ParseError::TokenExpected(
                    "Expected {".to_string()
                )
            )
        }

        // Confirmed an LBrace, so consume it
        self.advance()?;

        let mut body = vec![];

        // Keep consuming the tokens until we find a closing brace which will
        // indicate the end of the if statement
        loop {
            if self.is_at_end() {
                return Err(
                    ParseError::TokenExpected(
                        "}` expected before end of the file".to_string()
                    )
                );
            }

            // Got the }, so consume it so we finish the if statement and then break out of the loop
            if self.current()?.kind == TokenKind::RBrace {
                self.advance()?; // consume the }
                break;
            }

            // We have neither reached the end of the if statement or end of file, so keep parsing
            body.push(self.parse_statement()?)
        }

        Ok(
            Stmt::If {
                keyword: Keyword::If,
                condition,
                body,
            }
        )
    }

    /// Parse the section that the while loop should cover 
    fn parse_while_stmt(&mut self) -> Result<Stmt> {
        self.advance()?; // consume the while token

        // We should have an expression after while, so parse the condition
        let condition = self.parse_expression()?;

        // We need a { after the condition so make sure we have it
        if self.current()?.kind != TokenKind::LBrace {
            let lexeme = self.current()?.lexeme.to_string();
            return Err(
                ParseError::TokenExpected(
                    format!("Expected '{{' but got '{lexeme}' instead!")
                )
            )
        }

        // Confirmed that we have a {, so consume it
        self.advance()?;
        
        let mut body = vec![];
        
        loop {
            // A closing brace is a must to terminate the while loop
            if self.is_at_end() {
                return Err(
                    ParseError::TokenExpected(
                        "Expected '}' before end of file!".to_string()
                    )
                )
            }

            // Got the right brace, so consume and break out of the loop now
            if self.current()?.kind == TokenKind::RBrace {
                self.advance()?;
                break
            }

            // We have neither reached Eof, nor gotten }, so keep going
            body.push(self.parse_statement()?)
        }

        Ok(
            Stmt::While {
                keyword: Keyword::While,
                condition,
                body
            }
        )

    }
}

/// Private parsing methods for expressions
impl<'a> Parser<'a> {
    /// Start the parsing by calling the operation parser with least precedence;
    /// here, we called conditional cuz binary operations need to be done before
    /// conditional operations
    fn parse_expression(&mut self) -> Result<Expr> {
        self.parse_conditional()
    }

    /// Since we need to make sure to parse the binary operations if there are any,
    /// we call them from inside the conditional parser
    /// 
    /// If there are any binary operators, we would get binary tree,
    /// and if there are no binary operators, we would just get the identifiers or 
    /// numbers
    fn parse_conditional(&mut self) -> Result<Expr> {
        // get the expression to put at the left of the conditional operator 
        let mut left = self.parse_addition()?; 

        // If we haven't reached EOF yet (which is indicated by Ok()), check if 
        // there are any conditional operators, if there are, get the operator
        // and move forward, if not, break the loop and just return the binary tree
        // or identifier or number we had gotten from the first time we called
        // self.parse_addition()?; 
        while let Ok(current) = self.current() {
            let operator = match current.kind {
                TokenKind::EqualsEquals => ConditionalOp::IsEqualTo,
                TokenKind::GreaterThan => ConditionalOp::IsGreaterThan,
                TokenKind::LessThan => ConditionalOp::IsLessThan,
                TokenKind::GreaterOrEquals => ConditionalOp::IsGreaterOrEqualTo,
                TokenKind::LessOrEquals => ConditionalOp::IsLessOrEqualTo,
                TokenKind::ExEquals => ConditionalOp::IsNotEqualThan,
                _ => break
            };

            // We reaching this line means we have gotten a conditional operator,
            // so consume it
            self.advance()?;

            // Now parse the right hand side expression (which will give either a
            // number or identifier or a binary)
            let right = self.parse_addition()?;

            left = Expr::Conditional {
                left: Box::new(left),
                operator,
                right: Box::new(right)
            };
        }
        Ok(left)
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

        // println!("Current: {:#?}", token);
            
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
                // println!("returned the identifier");
                Ok(
                    Expr::Identifier(
                        self.advance()?.lexeme.to_string()
                    )
                )
            },
            TokenKind::String => {
                Ok(
                    Expr::String(
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
            _ => {
                println!("From primary");
                Err(
                    ParseError::InvalidToken(
                    token.lexeme.to_string()
                ))
            }
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
