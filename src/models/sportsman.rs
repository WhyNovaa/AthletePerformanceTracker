use crate::models::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Sportsman(String);

impl Sportsman {
    pub fn new(name: String) -> Result<Self, Error> {
        if name.len() > 50 {
            return Err(Error::NameTooLong);
        }
        Ok(Self(name))
    }

    pub fn unchecked_new(name: String) -> Self {
        Self(name)
    }
    pub fn name(&self) -> String {
        self.0.clone()
    }
}

impl fmt::Display for Sportsman {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Name: {}", self.0)?;
        Ok(())
    }
}
