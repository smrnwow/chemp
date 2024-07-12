/// An enumeration of parsing errors
#[derive(Debug)]
pub enum Error {
    UnexpectedCharacter(char),
    UnexpectedEOF,
    UnknownElement(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnexpectedCharacter(character) => {
                write!(f, "unexpected character: {}", character)
            }

            Self::UnexpectedEOF => {
                write!(f, "unexpected end of formula")
            }

            Self::UnknownElement(element) => {
                write!(f, "unknown element: {}", element)
            }
        }
    }
}

impl std::error::Error for Error {}
