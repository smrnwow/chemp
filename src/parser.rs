use crate::chemistry::Table;
use crate::tokenizer::{Token, Tokenizer};
use crate::tokens::{Component, Element, Group, Hydrate, Substance};
use crate::Error;

pub struct Parser<'a> {
    table: &'a Table<'a>,
    tokenizer: Tokenizer<'a>,
    lookahead: Option<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(table: &'a Table, formula: &'a str) -> Self {
        Self {
            table,
            tokenizer: Tokenizer::new(formula),
            lookahead: None,
        }
    }

    pub fn parse(&mut self) -> Result<Substance, Error> {
        self.lookahead = self.tokenizer.next_token()?;

        self.substance()
    }

    fn substance(&mut self) -> Result<Substance, Error> {
        let mut substance = Substance::new();

        substance.add_coefficient(self.coefficient()?);

        while let Some(component) = self.component()? {
            substance.add_component(component);
        }

        if let Some(Token::Asterisk) = self.peek() {
            substance.add_hydrate(self.hydrate()?);
        }

        match self.peek() {
            Some(Token::Asterisk) => {
                substance.add_hydrate(self.hydrate()?);
            }

            Some(token) => {
                return Err(Error::UnexpectedToken(
                    token.value().to_string(),
                    self.tokenizer.cursor(),
                ));
            }

            None => {}
        }

        Ok(substance)
    }

    fn component(&mut self) -> Result<Option<Component>, Error> {
        let component = match self.peek() {
            Some(Token::LParen) => Some(Component::Group(self.group()?)),
            Some(Token::Symbol(value)) => Some(Component::Element(self.element(value)?)),
            _ => None,
        };

        Ok(component)
    }

    fn group(&mut self) -> Result<Group, Error> {
        self.consume(Token::LParen)?;

        let mut group = Group::new();

        while let Some(component) = self.component()? {
            group.add_component(component);
        }

        self.consume(Token::RParen)?;

        group.add_subscript(self.subscript()?);

        return Ok(group);
    }

    fn element(&mut self, value: &'a str) -> Result<Element, Error> {
        let symbol = self.consume(Token::Symbol(value))?;

        let chemical_element = match self.table.lookup(symbol.value()) {
            Some(chemical_element) => chemical_element,
            None => {
                return Err(Error::UnknownElement(
                    symbol.value().to_string(),
                    self.tokenizer.cursor(),
                ))
            }
        };

        let subscript = self.subscript()?;

        return Ok(Element::new(chemical_element, subscript));
    }

    fn hydrate(&mut self) -> Result<Hydrate, Error> {
        self.consume(Token::Asterisk)?;

        let mut hydrate = Hydrate::new();

        hydrate.add_coefficient(self.coefficient()?);

        self.consume(Token::Symbol("H"))?;

        self.consume(Token::Number("2"))?;

        self.consume(Token::Symbol("O"))?;

        Ok(hydrate)
    }

    fn coefficient(&mut self) -> Result<usize, Error> {
        let coefficient = match self.peek() {
            Some(Token::Number(value)) => {
                let number = self.consume(Token::Number(value))?;

                number.value().parse().unwrap()
            }

            _ => 1,
        };

        Ok(coefficient)
    }

    fn subscript(&mut self) -> Result<usize, Error> {
        let subscript = match self.peek() {
            Some(Token::Number(value)) => {
                let number = self.consume(Token::Number(value))?;

                number.value().parse().unwrap()
            }

            _ => 1,
        };

        Ok(subscript)
    }

    fn peek(&self) -> Option<&Token<'a>> {
        match &self.lookahead {
            Some(token) => Some(token),
            None => None,
        }
    }

    fn consume(&mut self, token: Token<'a>) -> Result<Token<'a>, Error> {
        let result = match &self.lookahead {
            Some(next_token) => {
                if *next_token == token {
                    Ok(*next_token)
                } else {
                    Err(Error::UnexpectedToken(
                        next_token.value().to_string(),
                        self.tokenizer.cursor(),
                    ))
                }
            }

            None => Err(Error::UnexpectedEnd(token.value().to_string())),
        };

        if result.is_ok() {
            self.lookahead = self.tokenizer.next_token()?;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::chemistry::Table;
    use crate::tokens::{Component, Element, Group, Hydrate, Substance};

    #[test]
    fn parser_single_element() {
        let table = Table::new();

        assert_eq!(
            Parser::new(&table, "N").parse().unwrap(),
            Substance::from(1, vec![Component::Element(Element::from("N", 1))], None)
        );

        assert_eq!(
            Parser::new(&table, "Mg").parse().unwrap(),
            Substance::from(1, vec![Component::Element(Element::from("Mg", 1))], None)
        );

        assert_eq!(
            Parser::new(&table, "Mg3").parse().unwrap(),
            Substance::from(1, vec![Component::Element(Element::from("Mg", 3))], None)
        );
    }

    #[test]
    fn parser_multiple_elements() {
        let table = Table::new();

        assert_eq!(
            Parser::new(&table, "KNO3").parse().unwrap(),
            Substance::from(
                1,
                vec![
                    Component::Element(Element::from("K", 1)),
                    Component::Element(Element::from("N", 1)),
                    Component::Element(Element::from("O", 3)),
                ],
                None,
            )
        );
    }

    #[test]
    fn parser_group() {
        let table = Table::new();

        assert_eq!(
            Parser::new(&table, "Ca(NO3)2").parse().unwrap(),
            Substance::from(
                1,
                vec![
                    Component::Element(Element::from("Ca", 1)),
                    Component::Group(Group::from(
                        vec![
                            Component::Element(Element::from("N", 1)),
                            Component::Element(Element::from("O", 3)),
                        ],
                        2,
                    )),
                ],
                None,
            )
        );

        assert_eq!(
            Parser::new(&table, "C14H18N3O10Fe(NH4)2").parse().unwrap(),
            Substance::from(
                1,
                vec![
                    Component::Element(Element::from("C", 14)),
                    Component::Element(Element::from("H", 18)),
                    Component::Element(Element::from("N", 3)),
                    Component::Element(Element::from("O", 10)),
                    Component::Element(Element::from("Fe", 1)),
                    Component::Group(Group::from(
                        vec![
                            Component::Element(Element::from("N", 1)),
                            Component::Element(Element::from("H", 4)),
                        ],
                        2,
                    )),
                ],
                None,
            )
        );
    }

    #[test]
    fn parser_coefficient() {
        let table = Table::new();

        assert_eq!(
            Parser::new(&table, "2C14H18N3O10Fe(NH4)2").parse().unwrap(),
            Substance::from(
                2,
                vec![
                    Component::Element(Element::from("C", 14)),
                    Component::Element(Element::from("H", 18)),
                    Component::Element(Element::from("N", 3)),
                    Component::Element(Element::from("O", 10)),
                    Component::Element(Element::from("Fe", 1)),
                    Component::Group(Group::from(
                        vec![
                            Component::Element(Element::from("N", 1)),
                            Component::Element(Element::from("H", 4)),
                        ],
                        2,
                    )),
                ],
                None,
            )
        );
    }

    #[test]
    fn parser_hydrate() {
        let table = Table::new();

        assert_eq!(
            Parser::new(&table, "MgSO4*7H2O").parse().unwrap(),
            Substance::from(
                1,
                vec![
                    Component::Element(Element::from("Mg", 1)),
                    Component::Element(Element::from("S", 1)),
                    Component::Element(Element::from("O", 4)),
                ],
                Some(Hydrate::from(7)),
            )
        );
    }
}
