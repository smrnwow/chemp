use crate::chemistry::Element;

#[derive(Clone, Debug)]
pub struct Table {
    elements: Vec<Element>,
}

impl Table {
    pub fn new() -> Table {
        let table = Table {
            elements: vec![
                Element::Nitrogen,
                Element::Phosphorus,
                Element::Potassium,
                Element::Calcium,
                Element::Magnesium,
                Element::Sulfur,
                Element::Iron,
                Element::Zink,
                Element::Manganese,
                Element::Boron,
                Element::Copper,
                Element::Molybdenum,
                Element::Hydrogen,
                Element::Carbon,
                Element::Oxygen,
                Element::Sodium,
                Element::Aluminium,
                Element::Silicon,
                Element::Chlorine,
                Element::Cobalt,
            ],
        };

        table
    }

    pub fn lookup(&self, symbol: &str) -> Option<Element> {
        let element = self
            .elements
            .iter()
            .find(|element| element.symbol() == symbol);

        match element {
            Some(element) => Some(element.clone()),
            None => None,
        }
    }
}
