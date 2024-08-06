use crate::chemistry::{ChemicalElement, Table};
use crate::tokens::{Component, Element, Group, Hydrate, Substance};
use crate::Error;
use core::iter::Peekable;
use core::str::Chars;

/// A parser which takes raw formula and produces a substance item from it
///
/// It also takes a reference on periodic table to validate symbols of chemical elements found
///
/// Grammar:
/// substance = coefficient? component+ hydrate?
/// component = element | group
/// group = '(' component+ ')' subscript?
/// element = symbol subscript?
/// hydrate = '*' coefficient? water
/// symbol = uppercased | uppercased lowercased
/// subscript = digit+
/// coefficient = digit+
/// water = 'H2O'
/// uppercased = {'A'..'Z'}
/// lowercased = {'a'..'z'}
/// digit = '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
///
pub struct Parser<'a> {
    table: &'a Table<'a>,
    chars: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(table: &'a Table, formula: &'a str) -> Self {
        Self {
            table,
            chars: formula.chars().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Substance, Error> {
        self.substance()
    }

    fn substance(&mut self) -> Result<Substance, Error> {
        let mut substance = Substance::new();

        if let Some(coefficient) = self.digit() {
            substance.add_coefficient(coefficient);
        }

        while let Some(component) = self.component()? {
            substance.add_component(component);
        }

        if let Some(hydrate) = self.hydrate()? {
            substance.add_hydrate(hydrate);
        }

        Ok(substance)
    }

    fn component(&mut self) -> Result<Option<Component>, Error> {
        if let Some(group) = self.group()? {
            return Ok(Some(Component::Group(group)));
        }

        if let Some(element) = self.element()? {
            return Ok(Some(Component::Element(element)));
        }

        Ok(None)
    }

    fn group(&mut self) -> Result<Option<Group>, Error> {
        if let None = self.terminal('(') {
            return Ok(None);
        }

        let mut group = Group::new();

        while let Some(component) = self.component()? {
            group.add_component(component);
        }

        if let None = self.terminal(')') {
            return Err(Error::UnexpectedEndOfGroup);
        }

        if let Some(subscript) = self.digit() {
            group.add_subscript(subscript);
        }

        return Ok(Some(group));
    }

    fn element(&mut self) -> Result<Option<Element>, Error> {
        let symbol = match self.symbol() {
            Some(symbol) => symbol,
            None => return Ok(None),
        };

        let chemical_element = match self.table.lookup(&symbol.as_str()) {
            Some(chemical_element) => chemical_element,
            None => return Err(Error::UnknownElement(symbol)),
        };

        let subscript = match self.digit() {
            Some(subscript) => subscript,
            None => 1,
        };

        return Ok(Some(Element::new(chemical_element, subscript)));
    }

    fn hydrate(&mut self) -> Result<Option<Hydrate>, Error> {
        if let None = self.terminal('*') {
            return Ok(None);
        }

        let mut hydrate = Hydrate::new();

        if let Some(coefficient) = self.digit() {
            hydrate.add_coefficient(coefficient);
        }

        if let Some(element) = self.element()? {
            if element.chemical_element() != ChemicalElement::Hydrogen {
                return Err(Error::IncorrectHydrate);
            }

            if element.subscript() != 2 {
                return Err(Error::IncorrectHydrate);
            }

            hydrate.add_element(element);
        }

        if let Some(element) = self.element()? {
            if element.chemical_element() != ChemicalElement::Oxygen {
                return Err(Error::IncorrectHydrate);
            }

            if element.subscript() != 1 {
                return Err(Error::IncorrectHydrate);
            }

            hydrate.add_element(element);
        }

        Ok(Some(hydrate))
    }

    fn symbol(&mut self) -> Option<String> {
        let mut symbol = String::new();

        while let Some(&char) = self.chars.peek() {
            if char.is_alphabetic() {
                if char.is_uppercase() && symbol.len() == 0 {
                    symbol.push(char);
                    self.chars.next();
                    continue;
                }

                if char.is_lowercase() && symbol.len() > 0 {
                    symbol.push(char);
                    self.chars.next();
                    continue;
                }
            }

            break;
        }

        if symbol.len() > 0 {
            return Some(symbol);
        } else {
            return None;
        }
    }

    fn digit(&mut self) -> Option<usize> {
        let mut digit: String = String::new();

        while let Some(char) = self.chars.peek() {
            if char.is_digit(10) {
                digit.push(*char);
                self.chars.next();
            } else {
                break;
            }
        }

        if digit.len() > 0 {
            return Some(digit.parse().unwrap());
        } else {
            return None;
        }
    }

    fn terminal(&mut self, character: char) -> Option<char> {
        if let Some(&char) = self.chars.peek() {
            if char != character {
                return None;
            }

            self.chars.next();

            return Some(char);
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::chemistry::Table;
    use crate::tokens::{Component, Element, Group, Hydrate, Substance};

    #[test]
    fn single_element() {
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
    fn multiple_elements() {
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
    fn group() {
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
    fn coefficient() {
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
    fn hydrate() {
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
                Some(Hydrate::from(
                    7,
                    vec![Element::from("H", 2), Element::from("O", 1)],
                )),
            )
        );
    }
}
