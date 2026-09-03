// TODO: Add support for Token::Float, then update Parser
use super::LexError;

/// Terminal symbols produced by Lexer
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Strictly alphabetic identifiers
    Ident(String),
    Number(u32),

    Gt,
    Ge,
    Eq,
    Ne,
    Lt,
    Le,

    Or,
    And,

    LParen,
    RParen
}

#[derive(Default)]
pub struct Lexer {
    position: usize
}

impl Lexer {
    fn peek(&self, line: &str) -> Option<char> {
        line.get(self.position..)?.chars().next()
    }

    fn advance(&mut self, line: &str) -> Option<char> {
        let ch = self.peek(line)?;
        self.position += ch.len_utf8();
        Some(ch)
    }

    pub fn process_line(&mut self, line: &str) -> Result<Vec<Token>, LexError> {
        // Reset position
        self.position = 0;

        let mut tokens = Vec::new();

        while let Some(token) = self.next_token(line)? {
            tokens.push(token);
        }

        Ok(tokens)
    }

    fn next_token(&mut self, line: &str) -> Result<Option<Token>, LexError> {
        // Remove whitespace
        while let Some(c) = self.peek(line) {
            if c.is_whitespace() {
                self.advance(line);
            } else {
                break;
            }
        }

        // Check if end has been reached
        if self.position >= line.len() {
            return Ok(None)
        }

        // Safe to unwrap here
        let ch = self.peek(line).unwrap();

        // Ident
        if ch.is_ascii_alphabetic() {
            let mut ident = String::new();
            ident.push(ch);
            self.advance(line);

            while let Some(c) = self.peek(line) {
                if c.is_ascii_alphabetic() {
                    ident.push(c);
                    self.advance(line);
                } else {
                    break;
                }
            }

            return Ok(Some(Token::Ident(ident)))
        }

        // Number
        if ch.is_ascii_digit() {
            let mut number: u32 = 0;
            number = number * 10 + (ch as u32 - '0' as u32);
            self.advance(line);

            while let Some(c) = self.peek(line) {
                if c.is_ascii_digit() {
                    number = number * 10 + (c as u32 - '0' as u32);
                    self.advance(line);
                } else {
                    break;
                }
            }

            return Ok(Some(Token::Number(number)))
        }

        // Gt, Ge
        if ch == '>' {
            self.advance(line);

            if let Some(c) = self.peek(line) && c == '=' {
                self.advance(line);
                return Ok(Some(Token::Ge))
            } else {
                return Ok(Some(Token::Gt))
            }
        }

        // Lt, Le
        if ch == '<' {
            self.advance(line);

            if let Some(c) = self.peek(line) && c == '=' {
                self.advance(line);
                return Ok(Some(Token::Le))
            } else {
                return Ok(Some(Token::Lt))
            }
        }

        // Ne
        if ch == '!' {
            self.advance(line);

            if let Some(c) = self.peek(line) && c == '=' {
                self.advance(line);
                return Ok(Some(Token::Ne))
            } else {
                return Err(LexError::UnexpectedChar(ch))
            }
        }

        // Eq
        if ch == '=' {
            self.advance(line);
            return Ok(Some(Token::Eq))
        }

        // Or
        if ch == '|' {
            self.advance(line);
            return Ok(Some(Token::Or))
        }

        // And
        if ch == '&' {
            self.advance(line);
            return Ok(Some(Token::And))
        }

        // LParen
        if ch == '(' {
            self.advance(line);
            return Ok(Some(Token::LParen))
        }

        // RParen
        if ch == ')' {
            self.advance(line);
            return Ok(Some(Token::RParen))
        }

        // Token could not be created from input
        Err(LexError::UnexpectedChar(ch))
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_line() {
        let line = "pid = 10 & cpu > 20";
        let mut lexer = Lexer::default();
        let tokens = lexer.process_line(line).unwrap();

        assert!(!tokens.is_empty());
        assert_eq!(tokens[0], Token::Ident("pid".to_string()));
        assert_eq!(tokens[1], Token::Eq);
        assert_eq!(tokens[2], Token::Number(10));
        assert_eq!(tokens[3], Token::And);
        assert_eq!(tokens[4], Token::Ident("cpu".to_string()));
        assert_eq!(tokens[5], Token::Gt);
        assert_eq!(tokens[6], Token::Number(20));
    }

    #[test]
    fn test_invalid_line() {
        let line = "pid != 10 | cpu < 20 & mem ! 0";
        let mut lexer = Lexer::default();
        let tokens = lexer.process_line(line);
        assert!(tokens.is_err());
        assert!(matches!(tokens, Err(LexError::UnexpectedChar('!'))))
    }
}





