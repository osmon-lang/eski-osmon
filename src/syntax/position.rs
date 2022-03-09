use super::interner::{str, Name};
use std::{
    fmt::{Display, Error, Formatter},
    result::Result,
};

pub struct Position {
    pub line: u32,
    pub column: u32,
    pub file: Name,
}

impl Position {
    pub const fn new(name: Name, l: u32, c: u32) -> Position {
        Position {
            line: l,
            column: c,
            file: name,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}.{}:{}", str(self.file), self.line, self.column)
    }
}
