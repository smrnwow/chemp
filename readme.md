chemp is a tool for parsing chemical formulas.

takes molecular formula of substance. for given correct formula it extracts information about compound:
- chemical composition
- molar mass
- mass percent of each element in composition

##### Usage

```rust
use chemp;

let compound = chemp::parse("MgSO4*7H2O").unwrap();

// getters for each component of compound
compound.components().values().for_each(|component| {
    // get mass of all atoms of element in compound
    component.mass();

    // get percent of component mass to compound mass
    component.mass_percent();

    // get atoms count of element in compound
    component.atoms_count();

    // get chemical element symbol
    component.chemical_element().symbol();

    // get chemical element atomic weight
    component.chemical_element().atomic_weight();
});

// list of elements in order they parsed
// nested groups are flattened
compound.composition().iter().for_each(|element| {
    // get subscript of element
    element.subscript();

    // get chemical element symbol
    element.chemical_element().symbol();

    // get chemical element atomic weight
    element.chemical_element().atomic_weight();     
});

// get molar mass of compound
compound.molar_mass();

println!("compound: {:#?}", compound);

// compound: Compound {
//     composition: [
//         Element {
//             chemical_element: Magnesium,
//             subscript: 1,
//         },
//         Element {
//             chemical_element: Sulfur,
//             subscript: 1,
//         },
//         Element {
//             chemical_element: Oxygen,
//             subscript: 4,
//         },
//         Element {
//             chemical_element: Hydrogen,
//             subscript: 14,
//         },
//         Element {
//             chemical_element: Oxygen,
//             subscript: 7,
//         },
//     ],
//     components: {
//         "O": Component {
//             chemical_element: Oxygen,
//             atoms_count: 11,
//             mass_percent: 71.40498,
//         },
//         "S": Component {
//             chemical_element: Sulfur,
//             atoms_count: 1,
//             mass_percent: 13.007879,
//         },
//         "Mg": Component {
//             chemical_element: Magnesium,
//             atoms_count: 1,
//             mass_percent: 9.861401,
//         },
//         "H": Component {
//             chemical_element: Hydrogen,
//             atoms_count: 14,
//             mass_percent: 5.725739,
//         },
//     },
//     molar_mass: 246.466,
// }
