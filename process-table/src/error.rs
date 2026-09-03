use std::fmt;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    ParsingError(ParseError),
    LexingError(LexError),
    ColumnsError(ColumnError),
    BufferError(BufError),
    ProcessStatsError,
    DeserializeError,
    SerializeError,
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "{err}"),
            Self::ParsingError(err) => write!(f, "Parse Error: {err}"),
            Self::LexingError(err) => write!(f, "Lex Error: {err}"),
            Self::ColumnsError(err) => write!(f, "Colum Error: {err}"),
            Self::BufferError(err) => write!(f, "Buffer Error: {err}"),
            Self::ProcessStatsError => write!(f, "Process Stats Error"),
            Self::DeserializeError => write!(f, "Deserialize Error"),
            Self::SerializeError => write!(f, "SerializeError")
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self { Self::Io(err) }
}

impl From<toml::de::Error> for Error {
    fn from(_err: toml::de::Error) -> Self {
        Self::DeserializeError
    }
}

impl From<toml::ser::Error> for Error {
    fn from(_err: toml::ser::Error) -> Self {
        Self::SerializeError
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Self::ParsingError(err)
    }
}

impl From<LexError> for Error {
    fn from(err: LexError) -> Self {
        Self::LexingError(err)
    }
}

impl From<ColumnError> for Error {
    fn from(err: ColumnError) -> Self {
        Self::ColumnsError(err)
    }
}

impl From<BufError> for Error {
    fn from(err: BufError) -> Self {
        Self::BufferError(err)
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnknownField(String),
    ExpectedField,
    ExpectedOperator,
    ExpectedNumber,
    ExpectedRParen,
    ExpectedLParen,
    UnexpectedToken,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownField(s) => write!(f, "Unknown field: {s}"),
            Self::ExpectedField => write!(f, "Expected field"),
            Self::ExpectedOperator => write!(f, "Expected operator"),
            Self::ExpectedNumber => write!(f, "Expected number"),
            Self::ExpectedRParen => write!(f, "Expected right parent"),
            Self::ExpectedLParen => write!(f, "Expected left parent"),
            Self::UnexpectedToken => write!(f, "UnexpectedToken"),
        }
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug)]
pub enum LexError {
    UnexpectedChar(char)
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedChar(ch) => write!(f, "Unexpected char: {ch}")
        }
    }
}

#[derive(Debug)]
pub enum ColumnError {
    BadSelection(usize),
    BadCapacity(usize)
}

impl fmt::Display for ColumnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadSelection(visual_selection) => {
                write!(f, "Bad visual selection: {visual_selection}")
            }
            Self::BadCapacity(capacity) => {
                write!(f, "Bad capacity: {capacity}")
            }
        }
    }
}

#[derive(Debug)]
pub enum BufError {
    InsertPositionOutOfBounds(usize),
    NonAsciiInsertCh(char),
    NonAsciiInsertStr(String),
    InsertPositionNotOnCharBoundary(usize),
    RemovePositionOutOfBounds(usize),
    RemovePositionNotOnCharBoundary(usize),
    BadCapacity(usize),
}

impl fmt::Display for BufError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InsertPositionOutOfBounds(insert_position) => {
                write!(f, "Bad insert position: {insert_position}")
            }
            Self::NonAsciiInsertCh(ch) => {
                write!(f, "Non ascii char insert: {ch}")
            }
            Self::NonAsciiInsertStr(s) => {
                write!(f, "Non ascii string insert: {s}")
            }
            Self::InsertPositionNotOnCharBoundary(pos) => {
                write!(f, "Insert position not on char boundary: {pos}")
            }
            Self::RemovePositionOutOfBounds(pos) => {
                write!(f, "Remove position out of bounds: {pos}")
            }
            Self::RemovePositionNotOnCharBoundary(pos) => {
                write!(f, "Remove position not on char boundary: {pos}")
            }
            Self::BadCapacity(cap) => {
                write!(f, "Bad capacity value: {cap}")
            }
        }
    }
}
