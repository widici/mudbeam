use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Error {
    description: String
}

impl Error {
    pub fn new(description: String) -> Error {
        return Error { description }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description)?;
        Ok(())
    }
}