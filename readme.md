chemp is a tool for parsing chemical formulas.

takes molecular formula of substance. for given correct formula it extracts information about compound:
- chemical composition
- molar mass
- mass percent of each element in composition

# Usage

```rust
use chemp;

let compound = chemp::parse("MgSO4*7H2O").unwrap();

// get chemical composition
compound.composition().iter().for_each(|component| {
    // get mass of all atoms of element in compound
    component.mass();

    // get percent of component mass to compound mass
    component.mass_percent();
});

// get molar mass of compound
compound.molar_mass();

println!("compound: {:#?}", compound);

// compound: Compound {
//      composition: {
//          "Mg": Component {
//              element: Magnesium,
//              atoms_count: 1,
//              mass_percent: 9.861401,
//          },
//          "S": Component {
//              element: Sulfur,
//              atoms_count: 1,
//              mass_percent: 13.007879,
//          },
//          "O": Component {
//              element: Oxygen,
//              atoms_count: 11,
//              mass_percent: 71.40498,
//          },
//          "H": Component {
//              element: Hydrogen,
//              atoms_count: 14,
//              mass_percent: 5.725739,
//          },
//      },
//      molar_mass: 246.466,
// }