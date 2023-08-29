/* err.rs
Create a module in a file named err.rs which handles the boxing of errors.

This module must implement an enum called ParseErr which will take care of the parsing errors. It must have the following elements:

Empty
Malformed: which has a dynamic boxed error as element
A structure called ReadErr which will take care of the reading errors, with an element called child_err of type Box<dyn Error>.

For each data structure, you will have to implement a function called fmt for the Display trait. 
It should write the message "Fail to parse todo" in the case of any parsing error. Otherwise, it should write the message "Fail to read todo file".

For the Error trait, the following functions (methods) have to be implemented:

source which returns an Option with the error:

For the ReadErr, it must return the option with the error.
For the ParseErr, it will return an option which is None if the tasks are empty, and the error if the parsing is malformed. */

/* 
Expected Functions
For err.rs

use std::fmt;
use std::fmt::Display;
use std::error::Error;

pub enum ParseErr {
    // expected public fields
}

// required by error trait
impl Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    }
}

pub struct ReadErr {
    // expected public fields
}

// required by error trait
impl Display for ReadErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    }
}

impl Error for ParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {

    }
}

impl Error for ReadErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {

    }
} */

use std::fmt;
use std::fmt::Display;
use std::error::Error;

#[derive(Debug)]
pub enum ParseErr {
    // expected public fields
    Empty,
    Malformed(Box<dyn Error>),
}

// required by error trait

impl Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseErr::Empty => write!(f, "Fail to parses todo"),
            ParseErr::Malformed(e) => write!(f, "Fail to parses todo" ),
        }
    }
}

#[derive(Debug)]
pub struct ReadErr {
    // expected public fields
    pub child_err: Box<dyn Error>,
}

// required by error trait

impl Display for ReadErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fail to read todo file")
    }
}


impl Error for ParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseErr::Empty => None,
            ParseErr::Malformed(e) => Some(e.as_ref()),
        }
    }
}


impl Error for ReadErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.child_err.as_ref())
    }
}

