use crate::tokens::{Component, Element};

#[derive(Debug, PartialEq)]
pub struct Group {
    composition: Vec<Component>,
    subscript: usize,
}

impl Group {
    pub(crate) fn new() -> Self {
        Self {
            composition: vec![],
            subscript: 1,
        }
    }

    pub(crate) fn from(composition: Vec<Component>, subscript: usize) -> Self {
        Self {
            composition,
            subscript,
        }
    }

    pub(crate) fn add_element(&mut self, element: Element) {
        self.composition.push(Component::Element(element));
    }

    pub(crate) fn add_group(&mut self, group: Group) {
        self.composition.push(Component::Group(group));
    }

    pub(crate) fn add_subscript(&mut self, subscript: usize) {
        self.subscript = subscript;
    }

    pub(crate) fn elements(&self) -> Vec<Element> {
        let mut elements = vec![];

        self.composition
            .iter()
            .for_each(|component| match component {
                Component::Element(element) => {
                    elements.push(Element::multiply(element, self.subscript));
                }

                Component::Group(group) => {
                    group.elements().iter().for_each(|element| {
                        elements.push(Element::multiply(element, self.subscript));
                    });
                }
            });

        elements
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::{Component, Element, Group};

    #[test]
    fn subscript_multiplication() {
        let group = Group::from(
            vec![
                Component::Element(Element::from("N", 1)),
                Component::Element(Element::from("O", 3)),
            ],
            2,
        );

        assert_eq!(
            group.elements(),
            vec![Element::from("N", 2), Element::from("O", 6)]
        );
    }

    #[test]
    fn nested_group() {
        let group = Group::from(
            vec![
                Component::Element(Element::from("N", 1)),
                Component::Group(Group::from(
                    vec![Component::Element(Element::from("H", 2))],
                    2,
                )),
            ],
            2,
        );

        assert_eq!(
            group.elements(),
            vec![Element::from("N", 2), Element::from("H", 8)]
        );
    }
}
