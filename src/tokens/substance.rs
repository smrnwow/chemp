use crate::tokens::{Component, Group, Hydrate, Symbol};

#[derive(Debug, PartialEq)]
pub struct Substance {
    coefficient: usize,
    composition: Vec<Component>,
    hydrate: Option<Hydrate>,
}

impl Substance {
    pub fn new() -> Self {
        Self {
            coefficient: 1,
            composition: vec![],
            hydrate: None,
        }
    }

    pub fn from(coefficient: usize, composition: Vec<Component>, hydrate: Option<Hydrate>) -> Self {
        Self {
            coefficient,
            composition,
            hydrate,
        }
    }

    pub fn add_coefficient(&mut self, coefficient: usize) {
        self.coefficient = coefficient;
    }

    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.composition.push(Component::Symbol(symbol));
    }

    pub fn add_group(&mut self, group: Group) {
        self.composition.push(Component::Group(group));
    }

    pub fn add_hydrate(&mut self, hydrate: Hydrate) {
        self.hydrate = Some(hydrate);
    }

    pub fn symbols(&self) -> Vec<Symbol> {
        let mut symbols: Vec<Symbol> = Vec::new();

        self.composition
            .iter()
            .for_each(|component| match component {
                Component::Symbol(symbol) => {
                    symbols.push(Symbol::multiply(symbol, self.coefficient));
                }

                Component::Group(group) => {
                    group.symbols().iter().for_each(|symbol| {
                        symbols.push(Symbol::multiply(symbol, self.coefficient));
                    });
                }
            });

        if let Some(hydrate) = &self.hydrate {
            hydrate.symbols().iter().for_each(|symbol| {
                symbols.push(Symbol::multiply(symbol, self.coefficient));
            });
        }

        symbols
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::{Component, Hydrate, Substance, Symbol};

    #[test]
    fn coefficient_multiplication() {
        let substance = Substance::from(
            3,
            vec![
                Component::Symbol(Symbol::from("Mg", 1)),
                Component::Symbol(Symbol::from("S", 1)),
                Component::Symbol(Symbol::from("O", 4)),
            ],
            Some(Hydrate::from(
                7,
                vec![Symbol::from("H", 2), Symbol::from("O", 1)],
            )),
        );

        assert_eq!(
            substance.symbols(),
            vec![
                Symbol::from("Mg", 3),
                Symbol::from("S", 3),
                Symbol::from("O", 12),
                Symbol::from("H", 42),
                Symbol::from("O", 21),
            ]
        );
    }
}
