use crate::tokens::{Substance, Symbol};
use crate::Component;
use std::collections::HashMap;

/// A compound parsed from formula
///
/// Contains info about composition and molar mass of compound, defined by formula
#[derive(Clone, Debug, PartialEq)]
pub struct Compound {
    composition: HashMap<&'static str, Component>,
    molar_mass: f32,
}

impl Compound {
    pub(crate) fn new() -> Self {
        Self {
            composition: HashMap::new(),
            molar_mass: 0.0,
        }
    }

    /// get chemical composition
    pub fn composition(&self) -> Vec<Component> {
        self.composition.values().cloned().collect()
    }

    /// get molar mass of compound
    pub fn molar_mass(&self) -> f32 {
        self.molar_mass
    }

    fn add_symbol(&mut self, symbol: Symbol) {
        self.composition
            .entry(&symbol.element().symbol())
            .and_modify(|component| component.add_atoms(symbol.subscript() as usize))
            .or_insert(Component::from(symbol));

        self.molar_mass += symbol.element().atomic_weight() * symbol.subscript() as f32;
    }

    fn calculate_mass_percentage(&mut self) {
        self.composition
            .values_mut()
            .for_each(|component| component.calculate_mass_percent(self.molar_mass));
    }
}

impl From<Substance> for Compound {
    fn from(substance: Substance) -> Self {
        let mut compound = Self::new();

        substance.symbols().iter().for_each(|symbol| {
            compound.add_symbol(*symbol);
        });

        compound.calculate_mass_percentage();

        compound
    }
}

#[cfg(test)]
mod tests {
    use super::Compound;
    use crate::tokens::{Component, Hydrate, Substance, Symbol};

    #[test]
    fn molar_mass_calculation() {
        const MAGNESIUM_SULFATE_MOLAR_MASS: f32 = 246.466;

        let compound = Compound::from(Substance::from(
            1,
            vec![
                Component::Symbol(Symbol::from("Mg", 1)),
                Component::Symbol(Symbol::from("S", 1)),
                Component::Symbol(Symbol::from("O", 4)),
            ],
            Some(Hydrate::from(
                7,
                vec![Symbol::from("H", 2), Symbol::from("O", 1)],
            )),
        ));

        assert_eq!(compound.molar_mass(), MAGNESIUM_SULFATE_MOLAR_MASS);
    }
}
