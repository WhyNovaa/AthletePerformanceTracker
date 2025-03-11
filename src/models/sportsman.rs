use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Sportsman(String);

impl Sportsman {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

impl fmt::Display for Sportsman {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Name: {}", self.0)?;
        Ok(())
    }
}
