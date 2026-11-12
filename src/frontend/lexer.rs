
use super::{
  Token,
  TokenKind,
  BooleanVal,
  Keyword,
  LexedOutput,
  Span,
};

use crate::language::Type;

#[derive(Debug, PartialEq)]
pub enum LexError {
    InvalidCharacter {
        character: char,
        span: Span,
    },
    UnterminatedString {
        span: Span
    }
}

type Result<T> = std::result::Result<T, LexError>;

pub struct Lexer<'a> {
    source: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new instance of the lexer
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            position: 0
        }
    }

    /// Give the character in the current position of the lexer if it exists,
    /// if not, give None to indicate we are at EOF
    fn current(&self) -> Option<char> {
        // Suppose we are at position 2 of "hello"
        // And we want to get Some("l")
        // 
        // so calling source[position..] will give us characters
        // from position 2 onwards, "llo" in our case
        // 
        // And calling .chars() on it will turn it into an iterator
        // and then the .next() will actually give the next character
        // "l" in our case
        // 
        self.source[self.position..].chars().next()
    }

    /// Give me the next character from my current position in the lexer if it exists,
    /// if not, give None to indicate we are at EOF
    fn peek(&self) -> Option<char> {
        // Lets take the string "world" for example
        // And suppose our position is 2, so "r"
        // 
        // And we want to get Some("l")
        // 
        // When we call source[position..] on it, we get "rld" and 
        // an iterator when we call .chars() on it
        // 
        // When we call the first .next(), we get the next character in our variable
        // which is "r" but we dont want that cuz that's our current position
        // so we call .next() to get Some("l") which is what we wanted
        let mut chars = self.source[self.position..].chars();

        chars.next();
        chars.next()
    }

    /// Move me to the next position in the lexer if we aren't at EOF yet
    /// But if we called it at EOF, it's an error because there's no way
    /// to advance to from the current position
    fn advance(&mut self) {
        // Suppose we are at position 2 of "world", so "r"
        // And we called advance() on it, all we want it to change our position
        // to 3 which should now point to "l",
        // 
        // But we use .len_utf8() to avoid landing in the middle of utf8 encoding
        // and we take the length of current character to increase our position
        // by that much cuz indexing (our position) starts from 0 whereas length
        // starts from 1, so we want our position/index to be incremented by the
        // length of our character, so we can move our position equivalent to it's length
        
        // Get the length of the current character first
        let character = self.current().expect("Advance called at EOF");

        // Now change it's position by the length of the current utf8 encoded character
        // because calling a bare .len() might land us at the middle of the encoding
        // which we obviously dont want
        self.position += character.len_utf8();
    }    

    /// Take the TokenKind we have detected and our current position
    /// and give me the whole token struct
    fn token(&self, kind: TokenKind, start: usize) -> Token<'a> {
        Token {
            // the kind we got
            kind, 
            
            // the string slice from the start position to the current one
            lexeme: &self.source[start..self.position], 

            // the span of the tokenkind or lexeme whose token is this
            span: Span {
                start: start,
                end: self.position
            }
        }
    }

    /// If the current character is whitespace, keep advancing to the next position
    fn skip_whitespace(&mut self) {
        while let Some(current) = self.current() {
            if current.is_whitespace()
            && !matches!(current, '\n' | '\r') {
                self.advance()
            } else {
                break
            }
        }
    }

    // Skip anything irrelevant like line comments and whitespaces/newlines (for now)
    fn skip_ignored(&mut self) {
        loop {
            while let Some(character) = self.current() {
                if character.is_whitespace() {
                    self.advance();
                } else {
                    break;
                }
            }

            if self.current() == Some('/')
                && self.peek() == Some('/') {
                    self.advance();
                    self.advance();

                    self.consume_line_comment();
                    continue;
            }
            
            break;
        }
    }
    
    fn consume_line_comment(&mut self) {
        while let Some(character) = self.current() {
            if matches!(character, '\n' | '\r') {
                break;
            }
            self.advance();
        }
    }
    
    fn is_next_whitespace(&self) -> bool {
        while let Some(next) = self.peek() {
            if next.is_whitespace() {
                return true
            } else {
                return false
            }
        } 
        return false
    }

    /// Check if the current character is a valid identifier,
    /// If it is, keep consuming until you find what is not valid
    /// 
    /// It allows lowercase OR uppercase letters OR numbers OR underscore
    fn consume_identifier(&mut self) {
        while let Some(character) = self.current() {
            match character {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    self.advance()
                },
                _ => break
            }
        }
    }

    /// Check the string slice from the start position to the current
    /// position and see if we have consumed a keyword, if yes, return
    /// the appropriate keyword, if not, just return as Identifier
    fn keyword_or_identifier(&self, start: usize) -> TokenKind {
        match &self.source[start..self.position] {
            "let" => TokenKind::Keyword(Keyword::Let),
            "mut" => TokenKind::Keyword(Keyword::Mut),
            "for" => TokenKind::Keyword(Keyword::For),
            "while" => TokenKind::Keyword(Keyword::While),
            "fn" => TokenKind::Keyword(Keyword::Fn),
            "if" => TokenKind::Keyword(Keyword::If),
            "else" => TokenKind::Keyword(Keyword::Else),
            "return" => TokenKind::Keyword(Keyword::Return),

            // "String" => TokenKind::Keyword(Keyword::Type(Type::String)),
            "int" => TokenKind::Keyword(Keyword::Type(Type::Int)),
            // "float" => TokenKind::Keyword(Keyword::Type(Type::Float)),
            "bool" => TokenKind::Keyword(Keyword::Type(Type::Bool)),
            
            "true" => TokenKind::BooleanLiteral(BooleanVal::True),
            "false" => TokenKind::BooleanLiteral(BooleanVal::False),

            _ => TokenKind::Identifier
        }
    }
    
    /// Keep consuming/advancing until we keep getting numbers and
    /// stop the moment we get anything else
    fn consume_number(&mut self) -> TokenKind {
        let mut has_dot = false;
        while let Some(character) = self.current() {
            match character {
                '0'..='9' => self.advance(),
                '.' => {
                    if !has_dot {
                        has_dot = true;
                        self.advance();
                    } else {
                        // Stop consuming if we got second dot; it might be a range like for i in 1..2
                        break;
                    }
                } 
                _ => break
            }
        }

        if has_dot {
            TokenKind::FloatLiteral
        } else {
            TokenKind::IntegerLiteral
        }
    }

    /// Keep consuming until we find the given quote
    /// Here we dont consume the quotes because we trust the caller 
    /// that they have already consumed the quotes and will consume after
    /// we return
    /// 
    /// We consume just the stuff inside the quotes
    /// 
    /// Because if we were to consume it, we will put double double quotes
    /// in the lexeme of the token (cuz it's a string slice)
    fn consume_string(&mut self, quote: char, start: usize) -> Result<()> {
        while let Some(current) = self.current() {
            if current == quote {
                return Ok(());
            }

            self.advance();
        }

        // We ideally wont reach in this line but if we somehow
        // reach here, some problem has occured
        return Err(LexError::UnterminatedString { 
            span: Span {
                start,
                end: self.position
        }})
    }
    
    /// It returns the next token(struct) if it's available; whenever someone calls it
    /// Its callers dont have to think of whitespaces, raw characters or anything at all
    /// All they care about it that whenever they call it, they will get a valid token
    /// Or panic if the next character is an invalid token
    fn next_token(&mut self) -> Result<Token<'a>> {  
        self.skip_ignored();

        let start = self.position;

        let Some(character) = self.current() else {
            return Ok(self.token(TokenKind::Eof, start))
        };

        match character {
            'a'..='z' | 'A'..='Z' | '_' => {
                // Only allow an identifier to start from lowercase OR uppercase
                // letters or underscore and it's fine if there are numbers in between
                // them as long as an identifier doesn't START with numbers
                self.consume_identifier();
                Ok(self.token(
                    // either get a specific keyword or one identifier invariant
                    self.keyword_or_identifier(start),
                    start
                ))
            },
            '0'..='9' => {
                let kind = self.consume_number();
                Ok(self.token(kind, start))
            },
            '=' => {
                if self.peek() == Some('=') {
                    self.advance();
                    self.advance();
                    return Ok(self.token(TokenKind::EqualsEquals, start))
                }
                self.advance();
                Ok(self.token(TokenKind::Equals, start))
            },
            '>' => {
                if self.peek() == Some('=') {
                    self.advance();
                    self.advance();
                    return Ok(self.token(TokenKind::GreaterOrEquals, start))
                }
                self.advance();
                Ok(self.token(TokenKind::GreaterThan, start))
            },
            '<' => {
                if self.peek() == Some('=') {
                    self.advance();
                    self.advance();
                    return Ok(self.token(TokenKind::LessOrEquals, start))
                }
                self.advance();
                Ok(self.token(TokenKind::LessThan, start))
            }
            ')' => {
                self.advance();
                Ok(self.token(TokenKind::RParen, start))
            },
            '(' => {
                self.advance();
                Ok(self.token(TokenKind::LParen, start))
            },
            '{' => {
                self.advance();
                Ok(self.token(TokenKind::LBrace, start))
            },
            '}' => {
                self.advance();
                Ok(self.token(TokenKind::RBrace, start))
            },
            ';' => {
                self.advance();
                Ok(self.token(TokenKind::SemiColon, start))
            },
            ':' => {
                self.advance();
                Ok(self.token(TokenKind::Colon, start))
            },
            ',' => {
                self.advance();
                Ok(self.token(TokenKind::Comma, start))
            },
            '.' => {
                self.advance();
                Ok(self.token(TokenKind::Dot, start))
            },
            '!' => {
                if self.peek() == Some('=') {
                    self.advance();
                    self.advance();
                    return Ok(self.token(TokenKind::NotEquals, start))
                }
                self.advance();
                Ok(self.token(TokenKind::Not, start))
            },
            '"' => {
                // Consume the detected double quote and let it consume
                self.advance();
                self.consume_string('"', start)?;
                // It returns after it encountes that quote, so consume
                // that last quote
                // 
                // This way, we just store the string inside the quotes, not
                // the quotes themselves
                self.advance();
                Ok(self.token(TokenKind::StringLiteral, start))
            },
            '\'' => {
                self.advance();
                self.consume_string('\'', start)?;
                self.advance();
                Ok(self.token(TokenKind::StringLiteral, start))
            },
            '+' => {
                if self.peek() == Some('+'){
                    self.advance();
                    self.advance();
                    return Ok(self.token(TokenKind::PlusPlus, start))
                } else if self.peek() == Some('=') {
                    self.advance();
                    self.advance();
                    return Ok(self.token(TokenKind::PlusEquals, start))
                }
                self.advance();
                Ok(self.token(TokenKind::Plus, start))
            },
            '-' => {                
                if self.peek() == Some('-') {
                    self.advance();
                    self.advance();
                    return Ok(self.token(TokenKind::MinusMinus, start))
                } else if self.peek() == Some('>') {
                    self.advance();
                    self.advance();
                    return Ok(self.token(TokenKind::Arrow, start))
                } else if self.peek() == Some('=') {
                    self.advance();
                    self.advance();
                    return Ok(self.token(TokenKind::MinusEquals, start))
                }
                self.advance();
                Ok(self.token(TokenKind::Minus, start))
            },
            '*' => {
                self.advance();
                Ok(self.token(TokenKind::Star, start))
            },
            '/' => {
              self.advance();
              Ok(self.token(TokenKind::Slash, start))
            },
            '|' if self.peek() == Some('|') => {
                self.advance();
                self.advance();
                return Ok(self.token(TokenKind::OrOr, start))
            },
            '&' if self.peek() == Some('&') => {
                self.advance();
                self.advance();
                return Ok(self.token(TokenKind::AndAnd, start))
            },
            _ => return Err(LexError::InvalidCharacter { 
                character, 
                span: Span {
                    start,
                    end: self.position
                }
            })
        }
        
    }
}

/// The methods that lets the callers avoid the manual and boring works
impl<'a> Lexer<'a> {
    /// Automatically get the LexedOutput instead of having to manually
    /// loop and store the value in a variable etc
    pub fn lex(&mut self) -> Result<LexedOutput<'a>> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;

            if token.kind == TokenKind::Eof {
                break;
            }

            tokens.push(token)
        }

        Ok(LexedOutput { 
            tokens: tokens.into_boxed_slice() 
        })
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(tk) = self.next_token() {
            if tk.kind == TokenKind::Eof {
                None
            } else {
                Some(tk)
            }
        } else {
            None
        }
    }
}
