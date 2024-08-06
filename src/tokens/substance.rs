use crate::tokens::{Component, Element, Hydrate};

#[derive(Debug, PartialEq)]
pub struct Substance {
    coefficient: usize,
    composition: Vec<Component>,
    hydrate: Option<Hydrate>,
}

impl Substance {
    pub(crate) fn new() -> Self {
        Self {
            coefficient: 1,
            composition: vec![],
            hydrate: None,
        }
    }

    #[allow(unused)]
    pub(crate) fn from(
        coefficient: usize,
        composition: Vec<Component>,
        hydrate: Option<Hydrate>,
    ) -> Self {
        Self {
            coefficient,
            composition,
            hydrate,
        }
    }

    pub(crate) fn add_coefficient(&mut self, coefficient: usize) {
        self.coefficient = coefficient;
    }

    pub(crate) fn add_component(&mut self, component: Component) {
        self.composition.push(component);
    }

    pub(crate) fn add_hydrate(&mut self, hydrate: Hydrate) {
        self.hydrate = Some(hydrate);
    }

    pub(crate) fn elements(&self) -> Vec<Element> {
        let mut elements: Vec<Element> = Vec::new();

        self.composition
            .iter()
            .for_each(|component| match component {
                Component::Element(element) => {
                    elements.push(Element::multiply(element, self.coefficient));
                }

                Component::Group(group) => {
                    group.elements().iter().for_each(|element| {
                        elements.push(Element::multiply(element, self.coefficient));
                    });
                }
            });

        if let Some(hydrate) = &self.hydrate {
            hydrate.elements().iter().for_each(|element| {
                elements.push(Element::multiply(element, self.coefficient));
            });
        }

        elements
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::{Component, Element, Hydrate, Substance};

    #[test]
    fn coefficient_multiplication() {
        let substance = Substance::from(
            3,
            vec![
                Component::Element(Element::from("Mg", 1)),
                Component::Element(Element::from("S", 1)),
                Component::Element(Element::from("O", 4)),
            ],
            Some(Hydrate::from(
                7,
                vec![Element::from("H", 2), Element::from("O", 1)],
            )),
        );

        assert_eq!(
            substance.elements(),
            vec![
                Element::from("Mg", 3),
                Element::from("S", 3),
                Element::from("O", 12),
                Element::from("H", 42),
                Element::from("O", 21),
            ]
        );
    }
}
