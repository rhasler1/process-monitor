/*
 * S = { A }
 * P = { A->Aa, A->Ab, A->a, A->b, A->e }
 *
 * "Pid=10"
 * A = <start>
 * ' -> <word>"Pid"
 * ' -> <word>"Pid"<operator>"="
 * ' -> <word>"Pid"<operator>"="<number>"10"
 *
 * */

/*
 * TODO: Build a small recursive-descent parser
 *
 * */
/*
 * Grammar:
 * filter           ::= or_expression
 * or_expression    ::= and_expression ("OR" and_expression)*
 * and_expression   ::= primary ("AND" primary)*
 * primary          ::= "(" or_expression ")" | comparison
 * comparison       ::= field operator value
 * field            ::= IDENT
 * operator         ::= ">" | ">=" | "=" | "!=" | "<" | "<="
 * value            ::= NUMBER
 *
 * */

use crate::ProcessEntry;

use super::{ParseError, Token};

/// Grammar Operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Gt,
    Ge,
    Eq,
    Ne,
    Lt,
    Le,
}

impl Operator {
    fn compare(&self, lhs: &Value, rhs: &Value) -> bool {
        match (lhs, rhs) {
            (Value::Pid(lhs), Value::Pid(rhs)) => self.compare_values(lhs, rhs),
            (Value::Cpu(lhs), Value::Cpu(rhs)) => self.compare_values(lhs, rhs),
            (Value::Mem(lhs), Value::Mem(rhs)) => self.compare_values(lhs, rhs),

            // Different types can never meaningfully compare.
            _ => false,
        }
    }

    fn compare_values<T>(&self, lhs: &T, rhs: &T) -> bool
    where
        T: PartialOrd + PartialEq,
    {
        match self {
            Operator::Gt => lhs > rhs,
            Operator::Ge => lhs >= rhs,
            Operator::Eq => lhs == rhs,
            Operator::Ne => lhs != rhs,
            Operator::Lt => lhs < rhs,
            Operator::Le => lhs <= rhs,
        }
    }
}

/// Grammar Fields
#[derive(Debug, Clone, PartialEq)]
pub enum Field {
    Pid,
    Cpu,
    Mem,
}

/// Grammar Values
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Pid(u32),
    Cpu(f32),
    Mem(u64),
    Name(String)
}

/// Represents nonterminal/syntactic structures
#[derive(Debug, Clone, PartialEq)]
pub enum AST {
    Compare {
        field: Field,
        op: Operator,
        value: Value,
    },
    And(Box<AST>, Box<AST>),
    Or(Box<AST>, Box<AST>),
}

impl AST {
    pub fn matches(&self, row: &ProcessEntry) -> bool {
        match self {
            AST::Compare { field, op, value } => {
                let field_value = match field {
                    Field::Pid => Value::Pid(row.pid().as_u32()),
                    Field::Cpu => Value::Cpu(row.cpu().as_f32()),
                    Field::Mem => Value::Mem(row.mem().as_u64()),
                };

                op.compare(&field_value, value)
            }

            AST::And(left, right) => {
                left.matches(row) && right.matches(row)
            }

            AST::Or(left, right) => {
                left.matches(row) || right.matches(row)
            }
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize
}


impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        token
    }

    pub fn parse(&mut self) -> Result<AST, ParseError> {
        let filter = self.parse_or()?;

        if self.peek().is_some() {
            return Err(ParseError::UnexpectedToken);
        }

        Ok(filter)
    }

    fn parse_or(&mut self) -> Result<AST, ParseError> {
        let mut lhs = self.parse_and()?;

        while matches!(self.peek(), Some(Token::Or)) {
            self.next();

            let rhs = self.parse_and()?;

            lhs = AST::Or(Box::new(lhs), Box::new(rhs));
        }

        Ok(lhs)
    }

    fn parse_and(&mut self) -> Result<AST, ParseError> {
        let mut lhs = self.parse_primary()?;

        while matches!(self.peek(), Some(Token::And)) {
            self.next();

            let rhs = self.parse_primary()?;

            lhs = AST::And(Box::new(lhs), Box::new(rhs));
        }

        Ok(lhs)
    }

    fn parse_primary(&mut self) -> Result<AST, ParseError> {
        match self.peek() {
            Some(Token::LParen) => {
                self.next();

                let result = self.parse_or()?;

                match self.next() {
                    Some(Token::RParen) => Ok(result),
                    _ => Err(ParseError::ExpectedRParen),
                }
            }

            _ => self.parse_comparison(),
        }
    }

    fn parse_comparison(&mut self) -> Result<AST, ParseError> {
        let field = match self.next() {
            Some(Token::Ident(name)) => match name.as_str() {
                "pid" => Field::Pid,
                "cpu" => Field::Cpu,
                "mem" => Field::Mem,
                _ => return Err(ParseError::UnknownField(name)),
            },

            _ => return Err(ParseError::ExpectedField),
        };

        let op = match self.next() {
            Some(Token::Gt) => Operator::Gt,
            Some(Token::Ge) => Operator::Ge,
            Some(Token::Eq) => Operator::Eq,
            Some(Token::Ne) => Operator::Ne,
            Some(Token::Lt) => Operator::Lt,
            Some(Token::Le) => Operator::Le,
            _ => return Err(ParseError::ExpectedOperator),
        };

        let value = match (&field, self.next()) {
            (Field::Pid, Some(Token::Number(n))) => {
                Value::Pid(n)
            }

            (Field::Cpu, Some(Token::Number(n))) => {
                Value::Cpu(n as f32)
            }

            (Field::Mem, Some(Token::Number(n))) => {
                Value::Mem(n as u64)
            }

            (_, _) => return Err(ParseError::ExpectedNumber),
        };

        Ok(AST::Compare {
            field,
            op,
            value,
        })
    }

    /*fn parse_comparison(&mut self) -> Result<AST, ParseError> {
        let field = match self.next() {
            Some(Token::Ident(name)) => match name.as_str() {
                "pid" => Field::Pid,
                "cpu" => Field::Cpu,
                "mem" => Field::Mem,
                _ => return Err(ParseError::UnknownField(name)),
            },

            _ => return Err(ParseError::ExpectedField),
        };

        let op = match self.next() {
            Some(Token::Gt) => Operator::Gt,
            Some(Token::Ge) => Operator::Ge,
            Some(Token::Eq) => Operator::Eq,
            Some(Token::Ne) => Operator::Ne,
            Some(Token::Lt) => Operator::Lt,
            Some(Token::Le) => Operator::Le,
            _ => return Err(ParseError::ExpectedOperator),
        };

        let value = match self.next() {
            Some(Token::Number(n)) => Value::U32(n),
            _ => return Err(ParseError::ExpectedNumber),
        };

        Ok(AST::Compare {
            field,
            op,
            value,
        })
    }*/
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parser() {
        let tokens = vec![
            Token::Ident("pid".to_string()),
            Token::Eq,
            Token::Number(10),
            Token::And,
            Token::Ident("cpu".to_string()),
            Token::Ge,
            Token::Number(20)
        ];

        let mut parser = Parser {
            tokens,
            pos: 0
        };

        let filter = parser.parse().unwrap();

        assert_eq!(
            AST::And(
                Box::new(AST::Compare { field: Field::Pid, op: Operator::Eq, value: Value::Pid(10) }),
                Box::new(AST::Compare { field: Field::Cpu, op: Operator::Ge, value: Value::Cpu(20 as f32) })
            ),
            filter
        )
    }
}

