use crate::api::error::Error;
use std::fmt;
use std::fmt::Formatter;
use validator::ValidationError;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct Sportsman(String);

impl Sportsman {
    pub fn new(name: String) -> Result<Self, Error> {
        validate_sportsman_name(&name)?;

        Ok(Self(name))
    }

    pub fn unchecked_new(name: String) -> Self {
        Self(name)
    }
    pub fn name(&self) -> &String {
        &self.0
    }
}

impl fmt::Display for Sportsman {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Name: {}", self.0)?;
        Ok(())
    }
}

fn validate_sportsman_name(name: &str) -> Result<(), ValidationError> {
    if name.len() > 50 || name.is_empty() {
        return Err(ValidationError::new(
            "Sportsman's name is too long or too short",
        ));
    }

    if !name
        .chars()
        .all(|c| c.is_alphabetic() || c == ' ' || c == '-')
    {
        return Err(ValidationError::new("Wrong sportsman's name"));
    }

    Ok(())
}
