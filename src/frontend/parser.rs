
use super::{
    BinaryOp,
    ConditionalOp,
    Expr,
    Keyword,
    LexedOutput,
    Program,
    Stmt,
    Token,
    TokenKind,
    DataType,
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
    InvalidDataType(String),
    // UnexpectedToken(String),
    TokenExpected(String),
    NotAKeyword(String),
    KeywordNotSupported(String),
    // UnsupportedKeyword(String),
    StringToI64,
}
// .keyword()
type Result<T> = std::result::Result<T, ParseError>;

/// Public methods for the parser
impl<'a> Parser<'a> {
    /// The method that the caller needs to first call by passing a vector of
    /// tokens to later parse those tokens
    /// 
    /// Initialize the parser
    pub fn new(output: LexedOutput<'a>) -> Self {
        Self {
            tokens: output.tokens.to_vec(),
            position: 0
        }
    }

    /// Create an ast by calling the entry point parser function, which later
    /// leads to a recursive calling between itself, add operations parser,
    /// multiplication parser and primary parser which calls the add/multiplication
    /// parser or simply returns a number or identifier/variable by pattern matching
    pub fn parse(&mut self) -> Result<Program> {
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
            
        Ok(
            Program{
                ast: ast.into_boxed_slice()
            }
        )
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
            Keyword::Fn => {
                self.parse_function_stmt()
            },
            _ => return Err(
                ParseError::KeywordNotSupported(
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
                condition,
                body: body.into_boxed_slice(),
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
                condition,
                body: body.into_boxed_slice()
            }
        )

    }

    /// Function parser
    /// 
    /// > This doesn't support arguments yet, nor can you actually use the `return` keyword yet,
    /// > so the return indication is useless
    fn parse_function_stmt(&mut self) -> Result<Stmt> {
        self.advance()?; // Consume the fn token

        if self.current()?.kind != TokenKind::Identifier {
            let token = self.current()?.lexeme;
            return Err(
                ParseError::TokenExpected(
                    format!("Expected an identifier, found '{token}'")
                )
            )
        }

        let name = if let Ok(current) = self.current() {
            let curr = current.lexeme.to_string();
            
            if current.kind != TokenKind::Identifier {
                return Err(
                    ParseError::TokenExpected(
                        format!("Expected an identifier but found '{curr}' instead!")
                    )
                )
            }

            self.advance()?;

            curr
        } else {
            // We have no more tokens left
            return Err(
                ParseError::TokenExpected(
                    format!("Expected an identifier before end of file!")
                )
            )
        };
        
        if self.current()?.kind != TokenKind::LParen {
            let token = self.current()?.lexeme;
            return Err(
                ParseError::TokenExpected(
                    format!("Expected a '(', found '{token}'!")
                )
            )
        }

        self.advance()?; // consume the left parenthesis (

        if self.current()?.kind != TokenKind::RParen {
            let token = self.current()?.lexeme;
            return Err(
                ParseError::TokenExpected(
                    format!("Expected a ')', found '{token}'!")
                )
            )
        }

        self.advance()?; // consume the right parenthesis )

        if self.current()?.kind != TokenKind::Arrow {
            let token = self.current()?.lexeme;
            return Err(
                ParseError::TokenExpected(
                    format!("Expected a '->' after function declaration, found '{token}'!")
                )
            )
        }

        self.advance()?; // consume the arrow

        let return_type = self.parse_data_type();

        // If we didn't get None then it means there is 
        // a return type, so consume it, if not, we dont 
        // try to do so
        if return_type != None {
            self.advance()?;
        }
        
        if self.current()?.kind != TokenKind::LBrace {
            let token = self.current()?.lexeme;
            return Err(
                ParseError::TokenExpected(
                    format!("Expected a '{{', found '{token}'!")
                )
            )
        }

        self.advance()?; // consume the left brace {

        let mut body = vec![];
        let mut return_expr = None;
        
        loop {
            if let Some(ret_type) = return_type {
                if self.current()?.kind == TokenKind::Keyword(
                    Keyword::Return
                ) {
                    self.advance()?;
                    let expr_result = self.parse_expression()?;

                    // It's obvious in the case of number/integer and string
                    // 
                    // But which DataType do we need if we have an identifier
                    // expression?
                    // 
                    // Since identifier is not a data type, and same with 
                    match expr_result {
                        Expr::Number(_) => {
                            if !matches!(ret_type, DataType::Integer | DataType::Float) {
                                return Err(
                                    ParseError::InvalidDataType(
                                        format!("Expected an integer or a float but got '{ret_type:#?}' instead!")
                                    )
                                )
                            }
                        },
                        Expr::String(_) => {
                            if ret_type != DataType::String {
                                return Err(
                                    ParseError::InvalidDataType(
                                        format!("Expected a string but got '{ret_type:#?}' instead!")
                                    )
                                )
                            }
                        },
                        _ => {},
                    }

                    return_expr = Some(expr_result)
                }
            }
            
            if self.is_at_end() {
                return Err(
                    ParseError::TokenExpected(
                        "Expected a '}' before end of the file!".to_string()
                    )
                )
            }

            if self.current()?.kind == TokenKind::RBrace {
                self.advance()?;
                break
            }

            body.push(self.parse_statement()?)
        }

        Ok(
            Stmt::Fn {
                name,
                return_type,
                return_expr,
                body: body.into_boxed_slice()
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
                TokenKind::NotEquals => ConditionalOp::IsNotEqualTo,
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
            TokenKind::DataType(data_type) => match data_type {
                DataType::Integer => {
                    if let Some(float) = parse_to_f64(token.lexeme) {
                        self.advance()?;
                        Ok(Expr::Number(float))
                    } else {
                        Err(ParseError::StringToI64)
                    }
                },
                DataType::String => {
                    Ok(
                        Expr::String(
                            self.advance()?.lexeme.to_string()
                        )
                    ) 
                },
                _ => return Err(
                    ParseError::InvalidDataType(
                        self.current()?.lexeme.to_string()
                    )
                )
            },
            TokenKind::Identifier => {
                // println!("returned the identifier");
                let identifier = self.advance()?.lexeme.to_string();

                
                if self.current()?.kind == TokenKind::LParen {
                    if self.peek()?.kind == TokenKind::RParen {
                        self.advance()?; // (
                        self.advance()?; // )
                        return Ok(
                            Expr::Function {
                                name: identifier
                            }
                        )
                    } else {
                        return Err(
                            ParseError::TokenExpected(
                                "Use a ')' to call the function expression".to_string()
                            )
                        )
                    }
                } else if self.current()?.kind == TokenKind::RParen {
                    return Err(
                        ParseError::TokenExpected(
                            "Got a ')'.. Use '()' instead if you want to call a function!".to_string()
                        )
                    )
                }
                
                Ok(
                    Expr::Identifier(
                        identifier
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
                        return Err(
                            ParseError::TokenExpected(
                            ")".to_string()
                        ))
                    }
                }
            },
            _ => {
                Err(
                    ParseError::InvalidToken(
                    token.lexeme.to_string()
                ))
            }
        }
    }
}

/// Private parsing methods for data types
impl<'a> Parser<'a> {
    /// If this returns a Some(), we know that there is 
    /// a data type to be returned,
    /// if we got a None, we know that it doesn't return
    /// anything
    fn parse_data_type(&mut self) -> Option<DataType> {        
        if let Ok(current) = self.current() {
            match current.kind {
                TokenKind::DataType(_type) => {
                    Some(_type)
                },
                _ => None
            }
        } else {
            None
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

/// An external helper for parsing strings to f64
fn parse_to_f64(input: &str) -> Option<f64> {
    if let Ok(integer) = input.parse::<f64>() {
        Some(integer)
    } else {
        None
    }
}
