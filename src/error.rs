/// An enumeration of parsing errors
#[derive(Debug)]
pub enum Error {
    UnexpectedToken(String, usize),
    UnexpectedEnd(String),
    UnknownElement(String, usize),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken(token, position) => {
                write!(
                    f,
                    "unexpected token: \"{}\" at position {}",
                    token, position
                )
            }

            Self::UnexpectedEnd(expected) => {
                write!(f, "unexpected end, expected: \"{}\"", expected)
            }

            Self::UnknownElement(element, position) => {
                write!(
                    f,
                    "unknown element: \"{}\" at position {}",
                    element, position
                )
            }
        }
    }
}

impl std::error::Error for Error {}
