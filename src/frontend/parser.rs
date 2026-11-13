

use crate::language::{
    BinaryOp,
    ConditionalOp,
    UnaryOp,
    UnaryFixity,
    AssignOp,
    Type,
};
use super::{
    Expr,
    FunctionParameter,
    Keyword,
    LexedOutput,
    Program,
    Stmt,
    Token,
    TokenKind,
    BooleanVal,
};

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: Box<[Token<'a>]>,
    position: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Eof,
    InvalidToken(String),
    Type(String),
    // UnexpectedToken(String),
    TokenExpected(String),
    NotAKeyword(String),
    KeywordNotSupported(String),
    UnsupportedConditionalStatement(String),
    // UnsupportedKeyword(String),
    StringToF64,
    StringToI64,
}

type Result<T> = std::result::Result<T, ParseError>;

/// Public methods for the parser
impl<'a> Parser<'a> {
    /// The method that the caller needs to first call by passing a vector of
    /// tokens to later parse those tokens
    /// 
    /// Initialize the parser
    pub fn new(output: LexedOutput<'a>) -> Self {
        Self {
            tokens: output.tokens,
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
    /// ```no_run
    /// let x = a + b
    /// if x > 10 {
    ///     // do something
    /// }
    /// ```
    fn parse_statement(&mut self) -> Result<Stmt> {   
        match self.current()?.kind {
            TokenKind::Keyword(Keyword::Let) => self.parse_let_stmt(),
            TokenKind::Keyword(Keyword::If) => self.parse_if_stmt(),
            TokenKind::Keyword(Keyword::While) => self.parse_while_stmt(),
            TokenKind::Keyword(Keyword::Fn) => self.parse_function_stmt(),
            TokenKind::Keyword(Keyword::Return) => self.parse_return_stmt(),
            
            TokenKind::Identifier => {
                if self.peek_assign_operator().is_some() {
                    self.parse_assign_stmt()
                } else {
                    Ok(Stmt::Expression(self.parse_expression()?))
                }
            },

            _ => {
                let curr_lexeme = self.curr_lexeme()?;
                return Err(ParseError::KeywordNotSupported(
                    format!("'{curr_lexeme}' is not a supported keyword!")
                ))
            }
        }
    }
    
    /// Parse the statement if it starts with let
    fn parse_let_stmt(&mut self) -> Result<Stmt> {  
        self.expect_keyword(Keyword::Let)?;

        let mut mutable = false;
        if self.take_if(TokenKind::Keyword(Keyword::Mut)) {
            mutable = true
        }
        let identifier = self.expect_identifier()?;
        
        let mut annotation = None;
        if self.take_if(TokenKind::Colon) {
            annotation = Some(self.expect_type()?)
        }
        
        self.expect(TokenKind::Equals)?;

        let expression = self.parse_expression()?;

        self.expect(TokenKind::SemiColon)?;
        
        Ok(Stmt::Let {
            identifier,
            mutable,
            annotation,
            expression,
        })
        
    }

    /// Parse the statement if it starts with an identifier
    /// which is likely to be an assignment statement
    fn parse_assign_stmt(&mut self) -> Result<Stmt> {
        let identifier = self.expect_identifier()?; // Consume the identifier

        let operator = self.expect_assign_operator()?;
        
        let expression = self.parse_expression()?;

        self.expect(TokenKind::SemiColon)?;
        
        Ok(
            Stmt::Assign {
                identifier,
                operator,
                expression
            }
        )
    }

    /// If and while statements have same 2 things: a conditional expression and
    /// a code block, so they can be parsed in the same way
    
    /// Parse the section of the tokens that come under the if statement
    fn parse_if_stmt(&mut self) -> Result<Stmt> {
        self.parse_condition_stmt(Keyword::If)
    }

    /// Parse the section that the while loop should cover 
    fn parse_while_stmt(&mut self) -> Result<Stmt> {
        self.parse_condition_stmt(Keyword::While)
    }

    /// Function parser
    /// A function statement is the declaration/implementation of a function; not the call to it.
    /// For example (statement)
    /// ```no_run
    /// fn add_one(a: int) -> int {
    ///     return a + 1
    /// }
    /// ```
    /// 
    /// The function expression looks like this
    /// ```no_run
    /// 
    /// let x = add_one(1) 
    /// ```
    fn parse_function_stmt(&mut self) -> Result<Stmt> {
        self.expect_keyword(Keyword::Fn)?;

        let name = self.expect_identifier()?;
        
        self.expect(TokenKind::LParen)?;

        let parameters = self.parse_comma_separated(|parser| {
            let name = parser.expect_identifier()?;
            parser.expect(TokenKind::Colon)?;

            let data_type = parser.expect_type()?;

            Ok(FunctionParameter {
                name,
                data_type
            })
        })?.into_boxed_slice();
        
        let mut return_type = Type::Unit;
        if self.take_if(TokenKind::Arrow) {
            return_type = self.expect_type()?;
        }
        
        self.expect(TokenKind::LBrace)?;

        let mut body = Vec::new();

        while !self.at(TokenKind::RBrace) {
            if self.is_at_end() {
                return Err(ParseError::TokenExpected(
                    "Expected '}' before end of file!".to_string()
                ));
            }

            body.push(self.parse_statement()?);
        }

        self.expect(TokenKind::RBrace)?;
        
        Ok(
            Stmt::Function {
                name,
                parameters,
                return_type,
                body: body.into_boxed_slice()
            }
        )
    }

    fn parse_return_stmt(&mut self) -> Result<Stmt> {
        self.expect(TokenKind::Keyword(Keyword::Return))?;

        let value = self.parse_expression()?;

        self.expect(TokenKind::SemiColon)?;
        
        Ok(
            Stmt::Return(Some(value))
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
         * Call parse_unary() to get the numbers/identifiers
         * or a binary (either containing numbers/identifiers or other binaries)
         */
        let mut left = self.parse_unary()?;

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

    fn parse_unary(&mut self) -> Result<Expr> {
        if let Some(operator) = self.current_prefix_operator() {
            self.advance()?;
            
            let operand = self.parse_unary()?;
            return Ok(Expr::Unary {
                operator,
                operand: Box::new(operand),
                fixity: UnaryFixity::Prefix
            });
        }
        self.parse_postfix()
    }


    fn parse_postfix(&mut self) -> Result<Expr> {
        let mut operand = self.parse_primary()?;

        while let Some(current) = self.current_kind() {
            let operator = match current {
                TokenKind::PlusPlus => UnaryOp::Increment,
                TokenKind::MinusMinus => UnaryOp::Decrement,
                _ => break
            };

            self.advance()?;

            operand = Expr::Unary {
                operator,
                operand: Box::new(operand),
                fixity: UnaryFixity::Postfix
            };
        }

        Ok(operand)
    }
    
    /*
     * This is the actual fundamental function that identifies what we have and returns
     * numbers or identifiers or expressions by pattern matching
     * Others just indirectly call it and add respective operators; they dont do the underlying work
     */
    fn parse_primary(&mut self) -> Result<Expr> {
        let token = self.current()?;
            
        match token.kind {
            TokenKind::IntegerLiteral => {
                if let Some(integer) = parse_to_i64(token.lexeme) {
                    self.advance()?;
                    Ok(Expr::Integer(integer))
                } else {
                    Err(ParseError::StringToI64)
                }
            },
            TokenKind::FloatLiteral => {
                if let Some(float) = parse_to_f64(token.lexeme) {
                    self.advance()?;
                    Ok(Expr::Float(float))
                } else {
                    Err(ParseError::StringToF64)
                }
            },
            TokenKind::StringLiteral => {
                Ok(Expr::String(self.advance()?.lexeme.to_string()))
            },
            TokenKind::BooleanLiteral(boolean_val) => {
                self.advance()?;
                Ok(Expr::Boolean(boolean_val))
            },
            TokenKind::Identifier => {
                let identifier = self.advance()?.lexeme.to_string();

                if self.is_at_end() || self.current()?.kind != TokenKind::LParen {
                    return Ok(Expr::Identifier(identifier));
                }
                
                self.advance()?; // consume '('

                let arguments = Vec::new();
                if self.take_if(TokenKind::RParen) {
                    return Ok(Expr::FunctionCall {
                        name: identifier,
                        arguments: arguments.into_boxed_slice()
                    });
                }
                
                Ok(Expr::FunctionCall {
                    name: identifier,
                    arguments: self.parse_comma_separated(|parser| {
                        Ok(parser.parse_expression()?)
                    })?.into_boxed_slice()
                })
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
                if current.kind != TokenKind::RParen {
                    let lex = current.lexeme;
                    return Err(ParseError::TokenExpected(
                               format!("Expected ')' but got '{lex} instead'")
                        ));
                }
 
                self.advance()?;
                Ok(expr)
            },
            _ => {
                return Err(
                    ParseError::InvalidToken(
                    token.lexeme.to_string()
                ));
            }
        }
    }
}

/// Major complimentary helpers method that do much of the work
impl<'a> Parser<'a> {
    fn parse_comma_separated<T, F>(
        &mut self,
        // The method to call to parse the item
        mut parse_item: F,
    ) -> Result<Vec<T>>
    where 
        F: FnMut(&mut Self) -> Result<T>,
    {
        let mut items = Vec::new();

        // We do take_if() instead of at() because we want to consume and return if
        // we are already at )
        // 
        // Another reason for us to take RParen instead of returning/letting the caller
        // consume is that the caller calls this method only if they have already found/consumed
        // LParen, so we dont make them think whether or not this method has finished parsing or not
        // 
        // We always finish everything inside (), consume the last ) and only return 
        if self.take_if(TokenKind::RParen) {
            return Ok(items);
        }

        loop {
            items.push(parse_item(self)?);

            if self.take_if(TokenKind::RParen) {
                break;
            }

            self.expect(TokenKind::Comma)?;

            if self.at(TokenKind::RParen) {
                return Err(ParseError::TokenExpected(
                    "Expected an item after ','".to_string()
                ));
            }
        }

        Ok(items)
    }

    /// Parse any code block typically enclosed between {}}
    fn parse_block(&mut self) -> Result<Box<[Stmt]>> {
        // The block needs to start with a {
        self.expect(TokenKind::LBrace)?;

        let mut stmts = Vec::new();

        // Keep going until we get the stop kind or }
        while !self.at(TokenKind::RBrace) {
            if self.is_at_end() {
                return Err(ParseError::TokenExpected(
                    "Expected '}' before end of file!".to_string()
                ));
            }

            stmts.push(self.parse_statement()?);
        }
        
        // We must get a } or stop_kind after we have finished parsing a block
        self.expect(TokenKind::RBrace)?;

        Ok(stmts.into_boxed_slice())
    }

    fn parse_condition_stmt(&mut self, keyword: Keyword) -> Result<Stmt> {
        self.expect_keyword(keyword)?;

        match keyword {
            Keyword::If => Ok(Stmt::If { 
                condition: self.parse_expression()?,
                body: self.parse_block()?
            }),

            Keyword::While => Ok(Stmt::While { 
                condition: self.parse_expression()?,
                body: self.parse_block()?
            }),

            _ => {
                return Err(ParseError::UnsupportedConditionalStatement(
                    "Unsupported conditional statement was found!\n Use if or while instead".to_string()
                ))
            }
        }
    }

    fn take_boolean_literal(&mut self) -> Result<BooleanVal> {
        match self.current()?.kind {
            TokenKind::BooleanLiteral(value) => {
                self.advance()?;
                Ok(value)
            }
            _ => Err(ParseError::TokenExpected(
                "Expected a token".to_string()
            ))
        }
    }
}

/// General token helper methods
impl<'a> Parser<'a> {
    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }
    
    /// Get the current but give None if there's non (eof)
    fn current_opt(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.position)
    }

    /// If there are some tokens left, we will most likely get an Ok(),
    /// from the get statement below
    fn current(&self) -> Result<&Token<'a>> {
        self.current_opt().ok_or(ParseError::Eof)
    }
    
    fn current_kind(&self) -> Option<TokenKind> {
        self.current_opt().map(|token| token.kind)
    }

    fn curr_lexeme(&self) -> Result<String> {
        Ok(self.current()?.lexeme.to_string())
    }

    fn peek_opt(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.position + 1)
    }

    fn peek_kind(&self) -> Option<TokenKind> {
        self.peek_opt().map(|token| token.kind)
    }

    /// Are we at this tokenkind? True if yes
    fn at(&self, kind: TokenKind) -> bool {
        self.current_kind() == Some(kind)
    }

    /// Advance/consume if the current kind is this
    fn take_if(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.position += 1;
            true
        } else {
            false
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
}

/// Specific token related helpers
impl<'a> Parser<'a> {
    /// We expect a certain kind.
    /// Consume if we got it; and throw an error if we didn't get it
    fn expect(&mut self, expected: TokenKind) -> Result<()> {
        if self.take_if(expected) {
            Ok(())
        } else if self.is_at_end() {
            Err(ParseError::TokenExpected(
                format!("Expected {expected:#?}, but reached end of file")
            ))
        } else {
            let actual = self.curr_lexeme()?;

            Err(ParseError::TokenExpected(
                format!("Expected {expected:#?}, got '{actual}'")
            ))
        }
    }

    /// If the current kind is an identifier (confirmed if expect() passed)
    /// we return the current lexeme; being sure it's an identifier
    fn expect_identifier(&mut self) -> Result<String> {
        if !self.at(TokenKind::Identifier) {
            return Err(ParseError::TokenExpected(
                "Expected an identifier".to_string()
            ));
        }

        let identifier = self.curr_lexeme()?;
        self.advance()?;

        Ok(identifier)
    }

    fn expect_keyword(&mut self, expected: Keyword) -> Result<()> {
        match self.current()?.kind {
            TokenKind::Keyword(keyword) if keyword == expected => {
                self.advance()?;
                Ok(())
            }

            TokenKind::Keyword(keyword) => {
                Err(ParseError::TokenExpected(
                    format!("Expected keyword '{expected:#?}', got '{keyword:#?}'")
                ))
            }

            _ => {
                let actual = self.curr_lexeme()?;

                Err(ParseError::TokenExpected(
                    format!("Expected keyword, got '{actual}'")
                ))
            }
        }
    }

    /// Keyword data type means int, float etc
    fn current_type(&self) -> Option<Type> {
        match self.current_kind()? {
            TokenKind::Keyword(Keyword::Type(_type)) => Some(_type),
            _ => None
        }
    }

    /// Consume the data type if present
    fn take_type(&mut self) -> Option<Type> {
        let dt = self.current_type()?;
        self.advance().ok();
        Some(dt)
    }

    fn take_assign_operator(&mut self) -> Option<AssignOp> {
        let op = self.current_assign_operator()?;
        self.advance().ok();
        Some(op)
    }

    fn current_prefix_operator(&self) -> Option<UnaryOp> {
        match self.current_kind()? {
            TokenKind::Plus => Some(UnaryOp::Plus),
            TokenKind::Minus => Some(UnaryOp::Minus),
            TokenKind::Not => Some(UnaryOp::Not),
            TokenKind::PlusPlus => Some(UnaryOp::Increment),
            TokenKind::MinusMinus => Some(UnaryOp::Decrement),
            _ => None
        }
    }

    fn current_assign_operator(&self) -> Option<AssignOp> {
        match self.current_kind()? {
            TokenKind::Equals => Some(AssignOp::Assign),
            TokenKind::PlusEquals => Some(AssignOp::AddAssign),
            TokenKind::MinusEquals => Some(AssignOp::SubtractAssign),
            _ => None
        }
    }

    fn peek_assign_operator(&self) -> Option<AssignOp> {
        match self.peek_kind()? {
            TokenKind::Equals => Some(AssignOp::Assign),
            TokenKind::PlusEquals => Some(AssignOp::AddAssign),
            TokenKind::MinusEquals => Some(AssignOp::SubtractAssign),
            _ => None
        }
    }

    fn curr_is_assign_operator(&self) -> bool {
        if self.current_assign_operator().is_some() {
            return true
        } else {
            false
        }
    }

    fn peek_is_assign_operator(&self) -> bool {
        if self.peek_assign_operator().is_some() {
            return true
        } else {
            false
        }
    }
    
    /// Consume and return the data type if we got it.
    /// But throw an error if there is no data type currently
    fn expect_type(&mut self) -> Result<Type> {
        self.take_type().ok_or_else(|| {
            ParseError::TokenExpected(
                "Expected a data type (eg: int, float, sting)".to_string()
            )
        })
    }

    
    fn expect_assign_operator(&mut self) -> Result<AssignOp> {
        self.take_assign_operator().ok_or_else(|| {
            ParseError::TokenExpected(
                "Expected an assign operator (eg: =, +=, -=)".to_string()
            )
        })
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

/// An external helper for parsing strings to i64
fn parse_to_i64(input: &str) -> Option<i64> {
    if let Ok(integer) = input.parse::<i64>() {
        Some(integer)
    } else {
        None
    }
}
