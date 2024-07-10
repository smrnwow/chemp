#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Element {
    Nitrogen,
    Phosphorus,
    Potassium,
    Calcium,
    Magnesium,
    Sulfur,
    Iron,
    Zink,
    Manganese,
    Boron,
    Copper,
    Molybdenum,
    Hydrogen,
    Carbon,
    Oxygen,
    Sodium,
    Aluminium,
    Silicon,
    Chlorine,
    Cobalt,
}

impl Element {
    pub fn atomic_weight(&self) -> f32 {
        match self {
            Self::Nitrogen => 14.0067,
            Self::Phosphorus => 30.974,
            Self::Potassium => 39.098,
            Self::Calcium => 40.078,
            Self::Magnesium => 24.305,
            Self::Sulfur => 32.06,
            Self::Iron => 55.845,
            Self::Zink => 65.38,
            Self::Manganese => 54.938,
            Self::Boron => 10.81,
            Self::Copper => 63.546,
            Self::Molybdenum => 95.95,
            Self::Hydrogen => 1.008,
            Self::Carbon => 12.011,
            Self::Oxygen => 15.999,
            Self::Sodium => 22.990,
            Self::Aluminium => 26.982,
            Self::Silicon => 28.085,
            Self::Chlorine => 35.45,
            Self::Cobalt => 58.933,
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Nitrogen => "N",
            Self::Phosphorus => "P",
            Self::Potassium => "K",
            Self::Calcium => "Ca",
            Self::Magnesium => "Mg",
            Self::Sulfur => "S",
            Self::Iron => "Fe",
            Self::Zink => "Zn",
            Self::Manganese => "Mn",
            Self::Boron => "B",
            Self::Copper => "Cu",
            Self::Molybdenum => "Mo",
            Self::Hydrogen => "H",
            Self::Carbon => "C",
            Self::Oxygen => "O",
            Self::Sodium => "Na",
            Self::Aluminium => "Al",
            Self::Silicon => "Si",
            Self::Chlorine => "Cl",
            Self::Cobalt => "Co",
        }
    }
}
