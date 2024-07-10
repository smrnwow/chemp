use crate::tokens::Symbol;

#[derive(Debug, PartialEq)]
pub struct Hydrate {
    coefficient: usize,
    symbols: Vec<Symbol>,
}

impl Hydrate {
    pub fn from(coefficient: usize, symbols: Vec<Symbol>) -> Self {
        Self {
            coefficient,
            symbols,
        }
    }

    pub fn new() -> Self {
        Self {
            coefficient: 1,
            symbols: vec![],
        }
    }

    pub fn add_coefficient(&mut self, coefficient: usize) {
        self.coefficient = coefficient;
    }

    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.push(symbol);
    }

    pub fn symbols(&self) -> Vec<Symbol> {
        self.symbols
            .iter()
            .map(|symbol| Symbol::multiply(symbol, self.coefficient))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::{Hydrate, Symbol};

    #[test]
    fn multiplication_coefficient() {
        let hydrate = Hydrate::from(7, vec![Symbol::from("H", 2), Symbol::from("O", 1)]);

        assert_eq!(
            hydrate.symbols(),
            vec![Symbol::from("H", 14), Symbol::from("O", 7)]
        );
    }
}
