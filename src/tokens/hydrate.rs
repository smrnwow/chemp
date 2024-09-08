use crate::chemistry::ChemicalElement;
use crate::tokens::Element;

#[derive(Debug, PartialEq)]
pub struct Hydrate {
    coefficient: usize,
    elements: Vec<Element>,
}

impl Hydrate {
    pub(crate) fn new() -> Self {
        Self {
            coefficient: 1,
            elements: vec![
                Element::new(ChemicalElement::Hydrogen, 2),
                Element::new(ChemicalElement::Oxygen, 1),
            ],
        }
    }

    #[allow(unused)]
    pub(crate) fn from(coefficient: usize) -> Self {
        Self {
            coefficient,
            elements: vec![
                Element::new(ChemicalElement::Hydrogen, 2),
                Element::new(ChemicalElement::Oxygen, 1),
            ],
        }
    }

    pub fn add_coefficient(&mut self, coefficient: usize) {
        self.coefficient = coefficient;
    }

    pub fn elements(&self) -> Vec<Element> {
        self.elements
            .iter()
            .map(|element| Element::multiply(element, self.coefficient))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::{Element, Hydrate};

    #[test]
    fn multiplication_coefficient() {
        let hydrate = Hydrate::from(7);

        assert_eq!(
            hydrate.elements(),
            vec![Element::from("H", 14), Element::from("O", 7)]
        );
    }
}
