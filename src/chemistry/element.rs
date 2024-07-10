#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Element {
    Hydrogen,
    Helium,
    Lithium,
    Beryllium,
    Boron,
    Carbon,
    Nitrogen,
    Oxygen,
    Fluorine,
    Neon,
    Sodium,
    Magnesium,
    Aluminium,
    Silicon,
    Phosphorus,
    Sulfur,
    Chlorine,
    Argon,
    Potassium,
    Calcium,
    Scandium,
    Titanium,
    Vanadium,
    Chromium,
    Manganese,
    Iron,
    Cobalt,
    Nickel,
    Copper,
    Zinc,
    Gallium,
    Germanium,
    Arsenic,
    Selenium,
    Bromine,
    Krypton,
    Rubidium,
    Strontium,
    Yttrium,
    Zirconium,
    Niobium,
    Molybdenum,
    Technetium,
    Ruthenium,
    Rhodium,
    Palladium,
    Silver,
    Cadmium,
    Indium,
    Tin,
    Antimony,
    Tellurium,
    Iodine,
    Xenon,
    Caesium,
    Barium,
    Lanthanum,
    Cerium,
    Praseodymium,
    Neodymium,
    Promethium,
    Samarium,
    Europium,
    Gadolinium,
    Terbium,
    Dysprosium,
    Holmium,
    Erbium,
    Thulium,
    Ytterbium,
    Lutetium,
    Hafnium,
    Tantalum,
    Tungsten,
    Rhenium,
    Osmium,
    Iridium,
    Platinum,
    Gold,
    Mercury,
    Thallium,
    Lead,
    Bismuth,
    Polonium,
    Astatine,
    Radon,
    Francium,
    Radium,
    Actinium,
    Thorium,
    Protactinium,
    Uranium,
    Neptunium,
    Plutonium,
    Americium,
    Curium,
    Berkelium,
    Californium,
    Einsteinium,
    Fermium,
    Mendelevium,
    Nobelium,
    Lawrencium,
    Rutherfordium,
    Dubnium,
    Seaborgium,
    Bohrium,
    Hassium,
    Meitnerium,
    Darmstadtium,
    Roentgenium,
    Copernicium,
    Nihonium,
    Flerovium,
    Moscovium,
    Livermorium,
    Tennessine,
    Oganesson,
}

impl Element {
    pub fn atomic_weight(&self) -> f32 {
        match self {
            Self::Hydrogen => 1.008,
            Self::Helium => 4.002,
            Self::Lithium => 6.94,
            Self::Beryllium => 9.012,
            Self::Boron => 10.81,
            Self::Carbon => 12.011,
            Self::Nitrogen => 14.007,
            Self::Oxygen => 15.999,
            Self::Fluorine => 18.998,
            Self::Neon => 20.1797,
            Self::Sodium => 22.989,
            Self::Magnesium => 24.305,
            Self::Aluminium => 26.981,
            Self::Silicon => 28.085,
            Self::Phosphorus => 30.973,
            Self::Sulfur => 32.06,
            Self::Chlorine => 35.45,
            Self::Argon => 39.95,
            Self::Potassium => 39.0983,
            Self::Calcium => 40.078,
            Self::Scandium => 44.955,
            Self::Titanium => 47.867,
            Self::Vanadium => 50.9415,
            Self::Chromium => 51.9961,
            Self::Manganese => 54.938,
            Self::Iron => 55.845,
            Self::Cobalt => 58.933,
            Self::Nickel => 58.6934,
            Self::Copper => 63.546,
            Self::Zinc => 65.38,
            Self::Gallium => 69.723,
            Self::Germanium => 72.630,
            Self::Arsenic => 74.921,
            Self::Selenium => 78.971,
            Self::Bromine => 79.904,
            Self::Krypton => 83.798,
            Self::Rubidium => 85.4678,
            Self::Strontium => 87.62,
            Self::Yttrium => 88.905,
            Self::Zirconium => 91.224,
            Self::Niobium => 92.906,
            Self::Molybdenum => 95.95,
            Self::Technetium => 97.0,
            Self::Ruthenium => 101.07,
            Self::Rhodium => 102.905,
            Self::Palladium => 106.42,
            Self::Silver => 107.8682,
            Self::Cadmium => 112.414,
            Self::Indium => 114.818,
            Self::Tin => 118.710,
            Self::Antimony => 121.760,
            Self::Tellurium => 127.60,
            Self::Iodine => 126.904,
            Self::Xenon => 131.293,
            Self::Caesium => 132.905,
            Self::Barium => 137.327,
            Self::Lanthanum => 138.905,
            Self::Cerium => 140.116,
            Self::Praseodymium => 140.907,
            Self::Neodymium => 144.242,
            Self::Promethium => 145.0,
            Self::Samarium => 150.36,
            Self::Europium => 151.964,
            Self::Gadolinium => 157.25,
            Self::Terbium => 158.925,
            Self::Dysprosium => 162.500,
            Self::Holmium => 164.930,
            Self::Erbium => 167.259,
            Self::Thulium => 168.934,
            Self::Ytterbium => 173.045,
            Self::Lutetium => 174.9668,
            Self::Hafnium => 178.486,
            Self::Tantalum => 180.947,
            Self::Tungsten => 183.84,
            Self::Rhenium => 186.207,
            Self::Osmium => 190.23,
            Self::Iridium => 192.217,
            Self::Platinum => 195.084,
            Self::Gold => 196.966,
            Self::Mercury => 200.592,
            Self::Thallium => 204.38,
            Self::Lead => 207.2,
            Self::Bismuth => 208.980,
            Self::Polonium => 209.0,
            Self::Astatine => 210.0,
            Self::Radon => 222.0,
            Self::Francium => 223.0,
            Self::Radium => 226.0,
            Self::Actinium => 227.0,
            Self::Thorium => 232.0377,
            Self::Protactinium => 231.035,
            Self::Uranium => 238.028,
            Self::Neptunium => 237.0,
            Self::Plutonium => 244.0,
            Self::Americium => 243.0,
            Self::Curium => 247.0,
            Self::Berkelium => 247.0,
            Self::Californium => 251.0,
            Self::Einsteinium => 252.0,
            Self::Fermium => 257.0,
            Self::Mendelevium => 258.0,
            Self::Nobelium => 259.0,
            Self::Lawrencium => 262.0,
            Self::Rutherfordium => 267.0,
            Self::Dubnium => 270.0,
            Self::Seaborgium => 269.0,
            Self::Bohrium => 270.0,
            Self::Hassium => 270.0,
            Self::Meitnerium => 278.0,
            Self::Darmstadtium => 281.0,
            Self::Roentgenium => 281.0,
            Self::Copernicium => 285.0,
            Self::Nihonium => 286.0,
            Self::Flerovium => 289.0,
            Self::Moscovium => 289.0,
            Self::Livermorium => 293.0,
            Self::Tennessine => 293.0,
            Self::Oganesson => 294.0,
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Hydrogen => "H",
            Self::Helium => "He",
            Self::Lithium => "Li",
            Self::Beryllium => "Be",
            Self::Boron => "B",
            Self::Carbon => "C",
            Self::Nitrogen => "N",
            Self::Oxygen => "O",
            Self::Fluorine => "F",
            Self::Neon => "Ne",
            Self::Sodium => "Na",
            Self::Magnesium => "Mg",
            Self::Aluminium => "Al",
            Self::Silicon => "Si",
            Self::Phosphorus => "P",
            Self::Sulfur => "S",
            Self::Chlorine => "Cl",
            Self::Argon => "Ar",
            Self::Potassium => "K",
            Self::Calcium => "Ca",
            Self::Scandium => "Sc",
            Self::Titanium => "Ti",
            Self::Vanadium => "V",
            Self::Chromium => "Cr",
            Self::Manganese => "Mn",
            Self::Iron => "Fe",
            Self::Cobalt => "Co",
            Self::Nickel => "Ni",
            Self::Copper => "Cu",
            Self::Zinc => "Zn",
            Self::Gallium => "Ga",
            Self::Germanium => "Ge",
            Self::Arsenic => "As",
            Self::Selenium => "Se",
            Self::Bromine => "Br",
            Self::Krypton => "Kr",
            Self::Rubidium => "Rb",
            Self::Strontium => "Sr",
            Self::Yttrium => "Y",
            Self::Zirconium => "Zr",
            Self::Niobium => "Nb",
            Self::Molybdenum => "Mo",
            Self::Technetium => "Tc",
            Self::Ruthenium => "Ru",
            Self::Rhodium => "Rh",
            Self::Palladium => "Pd",
            Self::Silver => "Ag",
            Self::Cadmium => "Cd",
            Self::Indium => "In",
            Self::Tin => "Sn",
            Self::Antimony => "Sb",
            Self::Tellurium => "Te",
            Self::Iodine => "I",
            Self::Xenon => "Xe",
            Self::Caesium => "Cs",
            Self::Barium => "Ba",
            Self::Lanthanum => "La",
            Self::Cerium => "Ce",
            Self::Praseodymium => "Pr",
            Self::Neodymium => "Nd",
            Self::Promethium => "Pm",
            Self::Samarium => "Sm",
            Self::Europium => "Eu",
            Self::Gadolinium => "Gd",
            Self::Terbium => "Tb",
            Self::Dysprosium => "Dy",
            Self::Holmium => "Ho",
            Self::Erbium => "Er",
            Self::Thulium => "Tm",
            Self::Ytterbium => "Yb",
            Self::Lutetium => "Lu",
            Self::Hafnium => "Hf",
            Self::Tantalum => "Ta",
            Self::Tungsten => "W",
            Self::Rhenium => "Re",
            Self::Osmium => "Os",
            Self::Iridium => "Ir",
            Self::Platinum => "Pt",
            Self::Gold => "Au",
            Self::Mercury => "Hg",
            Self::Thallium => "Tl",
            Self::Lead => "Pb",
            Self::Bismuth => "Bi",
            Self::Polonium => "Po",
            Self::Astatine => "At",
            Self::Radon => "Rn",
            Self::Francium => "Fr",
            Self::Radium => "Ra",
            Self::Actinium => "Ac",
            Self::Thorium => "Th",
            Self::Protactinium => "Pa",
            Self::Uranium => "U",
            Self::Neptunium => "Np",
            Self::Plutonium => "Pu",
            Self::Americium => "Am",
            Self::Curium => "Cm",
            Self::Berkelium => "Bk",
            Self::Californium => "Cf",
            Self::Einsteinium => "Es",
            Self::Fermium => "Fm",
            Self::Mendelevium => "Md",
            Self::Nobelium => "No",
            Self::Lawrencium => "Lr",
            Self::Rutherfordium => "Rf",
            Self::Dubnium => "Db",
            Self::Seaborgium => "Sg",
            Self::Bohrium => "Bh",
            Self::Hassium => "Hs",
            Self::Meitnerium => "Mt",
            Self::Darmstadtium => "Ds",
            Self::Roentgenium => "Rg",
            Self::Copernicium => "Cn",
            Self::Nihonium => "Nh",
            Self::Flerovium => "Fl",
            Self::Moscovium => "Mc",
            Self::Livermorium => "Lv",
            Self::Tennessine => "Ts",
            Self::Oganesson => "Og",
        }
    }
}
