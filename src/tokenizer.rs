use crate::chemistry::Table;
use crate::tokens::{Element, Group, Hydrate, Substance};
use crate::Error;
use core::iter::Peekable;
use core::str::Chars;

pub struct Tokenizer<'a> {
    table: &'a Table<'a>,
    chars: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(table: &'a Table, formula: &'a str) -> Self {
        Self {
            table,
            chars: formula.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Substance, Error> {
        let mut empty = true;

        let mut substance = Substance::new();

        while let Some(&char) = self.chars.peek() {
            match char {
                '0'..='9' if empty => substance.add_coefficient(self.digit()),
                'A'..='Z' => substance.add_element(self.element()?),
                '(' => substance.add_group(self.group()?),
                '*' => substance.add_hydrate(self.hydrate()?),
                _ => return Err(Error::UnexpectedCharacter(char)),
            }

            if empty {
                empty = false;
            };
        }

        Ok(substance)
    }

    fn group(&mut self) -> Result<Group, Error> {
        self.chars.next();

        let mut group = Group::new();

        while let Some(&char) = self.chars.peek() {
            match char {
                'A'..='Z' => group.add_element(self.element()?),
                '(' => group.add_group(self.group()?),
                ')' => {
                    self.chars.next();
                    group.add_subscript(self.digit());
                    return Ok(group);
                }
                _ => return Err(Error::UnexpectedCharacter(char)),
            }
        }

        Err(Error::UnexpectedEOF)
    }

    fn hydrate(&mut self) -> Result<Hydrate, Error> {
        self.chars.next();

        let mut empty = true;

        let mut hydrate = Hydrate::new();

        while let Some(&char) = self.chars.peek() {
            match char {
                '0'..='9' if empty => hydrate.add_coefficient(self.digit()),
                'A'..='Z' => hydrate.add_element(self.element()?),
                _ => break,
            }

            if empty {
                empty = false;
            };
        }

        Ok(hydrate)
    }

    fn element(&mut self) -> Result<Element, Error> {
        let mut element = String::new();

        while let Some(&char) = self.chars.peek() {
            if char.is_alphabetic() {
                if char.is_uppercase() && element.len() == 0 {
                    element.push(char);
                    self.chars.next();
                    continue;
                }

                if char.is_lowercase() && element.len() > 0 {
                    element.push(char);
                    self.chars.next();
                    continue;
                }
            }

            break;
        }

        match self.table.lookup(&element.as_str()) {
            Some(chemical_element) => Ok(Element::new(chemical_element, self.digit())),
            None => Err(Error::UnknownElement(element)),
        }
    }

    fn digit(&mut self) -> usize {
        let mut digit: String = String::new();

        while let Some(&char) = self.chars.peek() {
            if char.is_digit(10) {
                digit.push(char);
                self.chars.next();
            } else {
                break;
            }
        }

        digit.parse().unwrap_or(1)
    }
}

#[cfg(test)]
mod tests {
    use super::Tokenizer;
    use crate::chemistry::Table;
    use crate::tokens::{Component, Element, Group, Hydrate, Substance};

    #[test]
    fn single_element() {
        let table = Table::new();

        assert_eq!(
            Tokenizer::new(&table, "N").tokenize().unwrap(),
            Substance::from(1, vec![Component::Element(Element::from("N", 1))], None)
        );

        assert_eq!(
            Tokenizer::new(&table, "Mg").tokenize().unwrap(),
            Substance::from(1, vec![Component::Element(Element::from("Mg", 1))], None)
        );

        assert_eq!(
            Tokenizer::new(&table, "Mg3").tokenize().unwrap(),
            Substance::from(1, vec![Component::Element(Element::from("Mg", 3))], None)
        );
    }

    #[test]
    fn multiple_elements() {
        let table = Table::new();

        assert_eq!(
            Tokenizer::new(&table, "KNO3").tokenize().unwrap(),
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
    fn group() {
        let table = Table::new();

        assert_eq!(
            Tokenizer::new(&table, "Ca(NO3)2").tokenize().unwrap(),
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
            Tokenizer::new(&table, "C14H18N3O10Fe(NH4)2")
                .tokenize()
                .unwrap(),
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
    fn coefficient() {
        let table = Table::new();

        assert_eq!(
            Tokenizer::new(&table, "2C14H18N3O10Fe(NH4)2")
                .tokenize()
                .unwrap(),
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
    fn hydrate() {
        let table = Table::new();

        assert_eq!(
            Tokenizer::new(&table, "MgSO4*7H2O").tokenize().unwrap(),
            Substance::from(
                1,
                vec![
                    Component::Element(Element::from("Mg", 1)),
                    Component::Element(Element::from("S", 1)),
                    Component::Element(Element::from("O", 4)),
                ],
                Some(Hydrate::from(
                    7,
                    vec![Element::from("H", 2), Element::from("O", 1)],
                )),
            )
        );
    }
}
