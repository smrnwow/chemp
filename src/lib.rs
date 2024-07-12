//! chemp is a tool for parsing chemical formulas.
//!
//! takes molecular formula of substance. for given correct formula it extracts information about compound:
//! - chemical composition
//! - molar mass
//! - mass percent of each element in composition
//!
//! # Usage
//!
//! ```rust
//! use chemp;
//!
//! let compound = chemp::parse("MgSO4*7H2O").unwrap();
//!
//! // get chemical composition
//! compound.composition().iter().for_each(|component| {
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
//!     component.element().symbol();
//!
//!     // get chemical element atomic weight
//!     component.element().atomic_weight();
//! });
//!
//! // get molar mass of compound
//! compound.molar_mass();
//!
//! println!("compound: {:#?}", compound);
//!
//! // compound: Compound {
//! //      composition: {
//! //          "Mg": Component {
//! //              element: Magnesium,
//! //              atoms_count: 1,
//! //              mass_percent: 9.861401,
//! //          },
//! //          "S": Component {
//! //              element: Sulfur,
//! //              atoms_count: 1,
//! //              mass_percent: 13.007879,
//! //          },
//! //          "O": Component {
//! //              element: Oxygen,
//! //              atoms_count: 11,
//! //              mass_percent: 71.40498,
//! //          },
//! //          "H": Component {
//! //              element: Hydrogen,
//! //              atoms_count: 14,
//! //              mass_percent: 5.725739,
//! //          },
//! //      },
//! //      molar_mass: 246.466,
//! // }

mod chemistry;
mod compounds;
mod error;
mod tokenizer;
mod tokens;

pub use chemistry::Element;
pub use compounds::{Component, Compound};
pub use error::Error;
use once_cell::sync::Lazy;
use tokenizer::Tokenizer;

static PERIODIC_TABLE: Lazy<chemistry::Table> = Lazy::new(|| chemistry::Table::new());

/// A function takes raw formula string and produce compound or error
pub fn parse<'a>(formula: impl Into<&'a str>) -> Result<Compound, Error> {
    let mut tokenizer = Tokenizer::new(&PERIODIC_TABLE, formula.into());

    let substance = tokenizer.tokenize()?;

    Ok(Compound::from(substance))
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::tokens::{Component, Hydrate, Substance, Symbol};
    use crate::Compound;

    #[test]
    fn simple() {
        let compound = parse("MgSO4*7H2O").unwrap();

        assert_eq!(
            compound,
            Compound::from(Substance::from(
                1,
                vec![
                    Component::Symbol(Symbol::from("Mg", 1)),
                    Component::Symbol(Symbol::from("S", 1)),
                    Component::Symbol(Symbol::from("O", 4)),
                ],
                Some(Hydrate::from(
                    7,
                    vec![Symbol::from("H", 2), Symbol::from("O", 1),]
                )),
            ))
        );
    }
}
