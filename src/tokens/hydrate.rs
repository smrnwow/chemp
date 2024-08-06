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
            elements: vec![],
        }
    }

    #[allow(unused)]
    pub(crate) fn from(coefficient: usize, elements: Vec<Element>) -> Self {
        Self {
            coefficient,
            elements,
        }
    }

    pub fn add_coefficient(&mut self, coefficient: usize) {
        self.coefficient = coefficient;
    }

    pub fn add_element(&mut self, element: Element) {
        self.elements.push(element);
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
        let hydrate = Hydrate::from(7, vec![Element::from("H", 2), Element::from("O", 1)]);

        assert_eq!(
            hydrate.elements(),
            vec![Element::from("H", 14), Element::from("O", 7)]
        );
    }
}
