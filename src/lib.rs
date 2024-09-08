//! chemp is a tool for parsing chemical formulas.
//!
//! takes molecular formula of substance. for given correct formula it extracts information about compound:
//! - chemical composition
//! - molar mass
//! - mass percent of each element in composition
//!
//! ##### Usage
//!
//! ```rust
//! use chemp;
//!
//! let compound = chemp::parse("MgSO4*7H2O").unwrap();
//!
//! // getters for each component of compound
//! compound.components().values().for_each(|component| {
//!     // get mass of all atoms of element in compound
//!     component.mass();
//!
//!     // get percent of component mass to compound mass
//!     component.mass_percent();
//!
//!     // get atoms count of element in compound
//!     component.atoms_count();
//!
//!     // get chemical element symbol
//!     component.chemical_element().symbol();
//!
//!     // get chemical element atomic weight
//!     component.chemical_element().atomic_weight();
//! });
//!
//! // get molar mass of compound
//! compound.molar_mass();
//!
//! println!("compound: {:#?}", compound);
//!
//! // compound: Compound {
//! //     components: {
//! //         "O": Component {
//! //             chemical_element: Oxygen,
//! //             atoms_count: 11,
//! //             mass_percent: 71.40498,
//! //         },
//! //         "S": Component {
//! //             chemical_element: Sulfur,
//! //             atoms_count: 1,
//! //             mass_percent: 13.007879,
//! //         },
//! //         "Mg": Component {
//! //             chemical_element: Magnesium,
//! //             atoms_count: 1,
//! //             mass_percent: 9.861401,
//! //         },
//! //         "H": Component {
//! //             chemical_element: Hydrogen,
//! //             atoms_count: 14,
//! //             mass_percent: 5.725739,
//! //         },
//! //     },
//! //     molar_mass: 246.466,
//! // }
//! ```

mod chemistry;
mod compounds;
mod error;
mod parser;
mod tokenizer;
mod tokens;

pub use chemistry::ChemicalElement;
pub use compounds::{Component, Compound};
pub use error::Error;
use once_cell::sync::Lazy;
use parser::Parser;
pub use tokens::Element;

static PERIODIC_TABLE: Lazy<chemistry::Table> = Lazy::new(|| chemistry::Table::new());

/// A function takes raw formula string and produce compound or error
pub fn parse<'a>(formula: impl Into<&'a str>) -> Result<Compound, Error> {
    let mut parser = Parser::new(&PERIODIC_TABLE, formula.into());

    let substance = parser.parse()?;

    Ok(Compound::from(substance))
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::tokens::{Component, Element, Hydrate, Substance};
    use crate::Compound;

    #[test]
    fn simple() {
        let compound = parse("MgSO4*7H2O").unwrap();

        assert_eq!(
            compound,
            Compound::from(Substance::from(
                1,
                vec![
                    Component::Element(Element::from("Mg", 1)),
                    Component::Element(Element::from("S", 1)),
                    Component::Element(Element::from("O", 4)),
                ],
                Some(Hydrate::from(7,)),
            ))
        );
    }
}
