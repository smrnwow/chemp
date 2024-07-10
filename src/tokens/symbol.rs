use crate::chemistry::{Element, Table};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Symbol {
    element: Element,
    subscript: usize,
}

impl Symbol {
    pub fn new(element: Element, subscript: usize) -> Self {
        Self { element, subscript }
    }

    pub fn from(symbol: &str, subscript: usize) -> Self {
        let table = Table::new();

        Self::new(table.lookup(symbol).unwrap(), subscript)
    }

    pub fn multiply(symbol: &Self, coefficient: usize) -> Self {
        Self {
            element: symbol.element(),
            subscript: symbol.subscript() * coefficient,
        }
    }

    pub fn element(&self) -> Element {
        self.element
    }

    pub fn subscript(&self) -> usize {
        self.subscript
    }

    pub fn molar_mass(&self) -> f32 {
        self.element.atomic_weight() * self.subscript as f32
    }
}

#[cfg(test)]
mod tests {
    use super::Symbol;
    use crate::chemistry::Table;

    #[test]
    fn multiplication() {
        let symbol = Symbol::from("K", 2);

        assert_eq!(Symbol::multiply(&symbol, 2).subscript(), 4);
    }

    #[test]
    fn molar_mass_calculation() {
        let table = Table::new();

        let element = table.lookup("K").unwrap();

        assert_eq!(
            Symbol::new(element, 2).molar_mass(),
            element.atomic_weight() * 2.
        );
    }
}
