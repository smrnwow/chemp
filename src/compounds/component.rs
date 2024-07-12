use crate::chemistry::ChemicalElement;
use crate::tokens::Element;

/// A component of compound
///
/// Contains info about detected chemical element, its atoms count and
/// calculated percent of molar mass of the compound
#[derive(Clone, Debug, PartialEq)]
pub struct Component {
    chemical_element: ChemicalElement,
    atoms_count: usize,
    mass_percent: f32,
}

impl Component {
    pub(crate) fn from(element: Element) -> Self {
        Self {
            chemical_element: element.chemical_element(),
            atoms_count: element.subscript() as usize,
            mass_percent: 0.0,
        }
    }

    pub(crate) fn add_atoms(&mut self, atoms_count: usize) {
        self.atoms_count += atoms_count;
    }

    pub(crate) fn calculate_mass_percent(&mut self, compound_mass: f32) {
        self.mass_percent = self.mass() / compound_mass * 100.;
    }

    /// get mass of all atoms of element in compound
    pub fn mass(&self) -> f32 {
        self.chemical_element.atomic_weight() * self.atoms_count as f32
    }

    /// get percent of component mass to compound mass
    pub fn mass_percent(&self) -> f32 {
        self.mass_percent
    }

    // get chemical element
    pub fn chemical_element(&self) -> ChemicalElement {
        self.chemical_element
    }

    // get atoms count of element in compound
    pub fn atoms_count(&self) -> usize {
        self.atoms_count
    }
}

#[cfg(test)]
mod tests {
    use super::{ChemicalElement, Component, Element};

    #[test]
    fn mass_calculation() {
        let mut component = Component::from(Element::from("S", 2));

        assert_eq!(
            component.mass(),
            ChemicalElement::Sulfur.atomic_weight() * 2.
        );

        component.add_atoms(3);

        assert_eq!(
            component.mass(),
            ChemicalElement::Sulfur.atomic_weight() * 5.
        );

        component.add_atoms(4);

        assert_eq!(
            component.mass(),
            ChemicalElement::Sulfur.atomic_weight() * 9.
        );
    }

    #[test]
    fn mass_percent_calculation() {
        const MAGNESIUM_SULFATE_MOLAR_MASS: f32 = 246.466;

        const SULFUR_PERCENTAGE_IN_MAGNESIUM_SULFATE: f32 = 13.007879;

        let mut component = Component::from(Element::from("S", 1));

        component.calculate_mass_percent(MAGNESIUM_SULFATE_MOLAR_MASS);

        assert_eq!(
            component.mass_percent(),
            SULFUR_PERCENTAGE_IN_MAGNESIUM_SULFATE
        );
    }
}
