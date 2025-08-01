use std::{error::Error, fmt::Display};
use std::fmt;

#[derive(Debug)]
pub enum ParseErr {
    Empty,
    Malformed( Box<dyn Error+ 'static>),
}

impl Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use the write! macro to format the output string
        write!(f, "Failed to parse todo file")
    }
}

impl Error for ParseErr {
    
}



#[derive(Debug)]
pub struct ReadErr {
    child_err: Box<dyn Error>,
}

impl Display for ReadErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use the write! macro to format the output string
        write!(f, "Failed to read todo file")
    }
}

impl Error for ReadErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.child_err)
    }
}
