use crate::tokens::{Component, Symbol};

#[derive(Debug, PartialEq)]
pub struct Group {
    composition: Vec<Component>,
    subscript: usize,
}

impl Group {
    pub fn from(composition: Vec<Component>, subscript: usize) -> Self {
        Self {
            composition,
            subscript,
        }
    }

    pub fn new() -> Self {
        Self {
            composition: vec![],
            subscript: 1,
        }
    }

    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.composition.push(Component::Symbol(symbol));
    }

    pub fn add_group(&mut self, group: Group) {
        self.composition.push(Component::Group(group));
    }

    pub fn add_subscript(&mut self, subscript: usize) {
        self.subscript = subscript;
    }

    pub fn symbols(&self) -> Vec<Symbol> {
        let mut symbols = vec![];

        self.composition
            .iter()
            .for_each(|component| match component {
                Component::Symbol(symbol) => {
                    symbols.push(Symbol::multiply(symbol, self.subscript));
                }

                Component::Group(group) => {
                    group.symbols().iter().for_each(|symbol| {
                        symbols.push(Symbol::multiply(symbol, self.subscript));
                    });
                }
            });

        symbols
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::{Component, Group, Symbol};

    #[test]
    fn subscript_multiplication() {
        let group = Group::from(
            vec![
                Component::Symbol(Symbol::from("N", 1)),
                Component::Symbol(Symbol::from("O", 3)),
            ],
            2,
        );

        assert_eq!(
            group.symbols(),
            vec![Symbol::from("N", 2), Symbol::from("O", 6)]
        );
    }

    #[test]
    fn nested_group() {
        let group = Group::from(
            vec![
                Component::Symbol(Symbol::from("N", 1)),
                Component::Group(Group::from(
                    vec![Component::Symbol(Symbol::from("H", 2))],
                    2,
                )),
            ],
            2,
        );

        assert_eq!(
            group.symbols(),
            vec![Symbol::from("N", 2), Symbol::from("H", 8)]
        );
    }
}
