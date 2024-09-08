use crate::tokens::{Element, Substance};
use crate::Component;
use std::collections::HashMap;

/// A compound parsed from formula
///
/// Contains info about composition and molar mass of compound, defined by formula
#[derive(Clone, Debug, PartialEq)]
pub struct Compound {
    components: HashMap<&'static str, Component>,
    molar_mass: f32,
}

impl Compound {
    pub(crate) fn new() -> Self {
        Self {
            components: HashMap::new(),
            molar_mass: 0.0,
        }
    }

    /// list components
    pub fn components(&self) -> &HashMap<&'static str, Component> {
        &self.components
    }

    /// get molar mass of compound
    pub fn molar_mass(&self) -> f32 {
        self.molar_mass
    }

    fn add_element(&mut self, element: Element) {
        self.components
            .entry(&element.chemical_element().symbol())
            .and_modify(|component| component.add_atoms(element.subscript()))
            .or_insert(Component::from(element));

        self.molar_mass += element.chemical_element().atomic_weight() * element.subscript() as f32;
    }

    fn calculate_mass_percentage(&mut self) {
        self.components
            .values_mut()
            .for_each(|component| component.calculate_mass_percent(self.molar_mass));
    }
}

impl From<Substance> for Compound {
    fn from(substance: Substance) -> Self {
        let mut compound = Self::new();

        substance.elements().iter().for_each(|element| {
            compound.add_element(*element);
        });

        compound.calculate_mass_percentage();

        compound
    }
}

#[cfg(test)]
mod tests {
    use super::Compound;
    use crate::tokens::{Component, Element, Hydrate, Substance};

    #[test]
    fn molar_mass_calculation() {
        const MAGNESIUM_SULFATE_MOLAR_MASS: f32 = 246.466;

        let compound = Compound::from(Substance::from(
            1,
            vec![
                Component::Element(Element::from("Mg", 1)),
                Component::Element(Element::from("S", 1)),
                Component::Element(Element::from("O", 4)),
            ],
            Some(Hydrate::from(7)),
        ));

        assert_eq!(compound.molar_mass(), MAGNESIUM_SULFATE_MOLAR_MASS);
    }
}
