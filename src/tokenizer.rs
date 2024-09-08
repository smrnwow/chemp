use crate::Error;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Token<'a> {
    Symbol(&'a str),
    Number(&'a str),
    LParen,
    RParen,
    Asterisk,
}

impl<'a> Token<'a> {
    pub fn value(&self) -> &'a str {
        match self {
            Self::Symbol(symbol) => symbol,
            Self::Number(number) => number,
            Self::LParen => "(",
            Self::RParen => ")",
            Self::Asterisk => "*",
        }
    }
}

pub struct Tokenizer<'a> {
    formula: &'a str,
    chars: Vec<char>,
    cursor: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(formula: &'a str) -> Self {
        Self {
            formula,
            chars: formula.chars().collect(),
            cursor: 0,
        }
    }

    pub fn next_token(&mut self) -> Result<Option<Token<'a>>, Error> {
        let token = match self.chars.get(self.cursor) {
            Some('A'..='Z') => self.symbol(),
            Some('1'..='9') => self.number(),
            Some(_) => match self.terminal() {
                Some(token) => Some(token),
                None => {
                    return Err(Error::UnexpectedToken(
                        self.chars.get(self.cursor).unwrap().to_string(),
                        self.cursor,
                    ));
                }
            },
            _ => None,
        };

        Ok(token)
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    fn symbol(&mut self) -> Option<Token<'a>> {
        let mut length = 0;

        while let Some(&char) = self.chars.get(self.cursor + length) {
            if char.is_alphabetic() {
                if char.is_uppercase() && length == 0 {
                    length += 1;
                    continue;
                }

                if char.is_lowercase() && length > 0 {
                    length += 1;
                    continue;
                }
            }

            break;
        }

        if length > 0 {
            let symbol = &self.formula[self.cursor..(self.cursor + length)];

            self.cursor += symbol.len();

            return Some(Token::Symbol(symbol));
        } else {
            return None;
        }
    }

    fn number(&mut self) -> Option<Token<'a>> {
        let mut length = 0;

        while let Some(char) = self.chars.get(self.cursor + length) {
            if char.is_digit(10) {
                length += 1;
                continue;
            } else {
                break;
            }
        }

        if length > 0 {
            let number = &self.formula[self.cursor..(self.cursor + length)];

            self.cursor += number.len();

            return Some(Token::Number(number));
        } else {
            return None;
        }
    }

    fn terminal(&mut self) -> Option<Token<'a>> {
        match self.chars.get(self.cursor) {
            Some(char) => {
                let token = match char {
                    '(' => Some(Token::LParen),
                    ')' => Some(Token::RParen),
                    '*' => Some(Token::Asterisk),
                    _ => None,
                };

                if token.is_some() {
                    self.cursor += 1;
                }

                token
            }

            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Token, Tokenizer};

    #[test]
    fn tokenizer_next_token() {
        let mut tokenizer = Tokenizer::new("Ca(NO3)2*2H2O");

        assert_eq!(tokenizer.next_token().unwrap(), Some(Token::Symbol("Ca")));

        assert_eq!(tokenizer.next_token().unwrap(), Some(Token::LParen));

        assert_eq!(tokenizer.next_token().unwrap(), Some(Token::Symbol("N")));

        assert_eq!(tokenizer.next_token().unwrap(), Some(Token::Symbol("O")));

        assert_eq!(tokenizer.next_token().unwrap(), Some(Token::Number("3")));

        assert_eq!(tokenizer.next_token().unwrap(), Some(Token::RParen));

        assert_eq!(tokenizer.next_token().unwrap(), Some(Token::Number("2")));

        assert_eq!(tokenizer.next_token().unwrap(), Some(Token::Asterisk));

        assert_eq!(tokenizer.next_token().unwrap(), Some(Token::Number("2")));

        assert_eq!(tokenizer.next_token().unwrap(), Some(Token::Symbol("H")));

        assert_eq!(tokenizer.next_token().unwrap(), Some(Token::Number("2")));

        assert_eq!(tokenizer.next_token().unwrap(), Some(Token::Symbol("O")));

        assert_eq!(tokenizer.next_token().unwrap(), None);

        assert_eq!(tokenizer.next_token().unwrap(), None);
    }
}
