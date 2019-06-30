use std::io::{self, Write};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    details: String
}

impl ParseError {
    fn new(msg: &str) -> ParseError {
        ParseError{details: msg.to_string()}
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.details
    }
}


pub fn parse_yes_no(s: String) -> Result<bool, ParseError> {
  if s == "yes" || s == "Yes" || s == "y" || s == "Y" {
    Ok(true)
  } else if s == "no" || s == "No" || s == "n" || s == "N" {
    Ok(false)
  } else {
    Err(ParseError::new("could not parse to bool"))
  }
}

pub fn ask_bool(question: &'static str) -> bool {
  loop {
    print!("{}", question);
    io::stdout().flush();
    match parse_yes_no(read!()) {
      Ok(res) => return res,
      Err(err) => print!("Could not parse your answer! \n {}", question),
    }
    io::stdout().flush();
  }
}
