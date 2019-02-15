use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct MappingError<'a> {
    pub why: &'a str,
}

impl<'a> Error for MappingError<'a> { }

impl<'a> Display for MappingError<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Mapping Error: {}", self.why)?;

        Ok(())
    }
}

impl<'a> Debug for MappingError<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Mapping Error: {}", self.why)?;

        Ok(())
    }
}