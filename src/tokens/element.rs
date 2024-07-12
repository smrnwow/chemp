use crate::chemistry::{ChemicalElement, Table};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Element {
    chemical_element: ChemicalElement,
    subscript: usize,
}

impl Element {
    pub(crate) fn new(chemical_element: ChemicalElement, subscript: usize) -> Self {
        Self {
            chemical_element,
            subscript,
        }
    }

    pub(crate) fn from(symbol: &str, subscript: usize) -> Self {
        let table = Table::new();

        Self::new(table.lookup(symbol).unwrap(), subscript)
    }

    pub(crate) fn multiply(symbol: &Self, coefficient: usize) -> Self {
        Self {
            chemical_element: symbol.chemical_element(),
            subscript: symbol.subscript() * coefficient,
        }
    }

    pub fn subscript(&self) -> usize {
        self.subscript
    }

    pub fn chemical_element(&self) -> ChemicalElement {
        self.chemical_element
    }
}

#[cfg(test)]
mod tests {
    use super::Element;

    #[test]
    fn multiplication() {
        let element = Element::from("K", 2);

        assert_eq!(Element::multiply(&element, 2).subscript(), 4);
    }
}
