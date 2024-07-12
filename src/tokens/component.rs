use crate::tokens::{Element, Group};

#[derive(Debug, PartialEq)]
pub enum Component {
    Element(Element),
    Group(Group),
}
