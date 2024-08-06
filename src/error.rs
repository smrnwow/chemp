/// An enumeration of parsing errors
#[derive(Debug)]
pub enum Error {
    UnexpectedCharacter(char),
    UnexpectedEOF,
    UnexpectedEndOfGroup,
    UnknownElement(String),
    IncorrectHydrate,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnexpectedCharacter(character) => {
                write!(f, "unexpected character: {}", character)
            }

            Self::UnexpectedEndOfGroup => {
                write!(f, "unexpected end of group")
            }

            Self::UnexpectedEOF => {
                write!(f, "unexpected end of formula")
            }

            Self::UnknownElement(element) => {
                write!(f, "unknown element: {}", element)
            }

            Self::IncorrectHydrate => {
                write!(f, "incorrect hydrate")
            }
        }
    }
}

impl std::error::Error for Error {}
