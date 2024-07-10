/// An enumeration of parsing errors
#[derive(Debug)]
pub enum Error {
    UnexpectedCharacter(char),
    UnexpectedEOF,
    UnknownSymbol(String),
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

            Self::UnknownSymbol(symbol) => {
                write!(f, "unknown symbol: {}", symbol)
            }
        }
    }
}

impl std::error::Error for Error {}
