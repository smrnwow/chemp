use crate::tokens::{Group, Symbol};

#[derive(Debug, PartialEq)]
pub enum Component {
    Symbol(Symbol),
    Group(Group),
}
