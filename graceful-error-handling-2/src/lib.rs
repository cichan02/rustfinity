use std::{
    error::Error,
    fmt::Display
};

// 1. Finish the definition
#[derive(Debug, PartialEq)]
pub enum ParsePercentageError {
    InvalidInput,
    OutOfRange
}

// 2. Implement the `Error` trait
impl Display for ParsePercentageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsePercentageError::InvalidInput => write!(f, "Input is not a valid number"),
            ParsePercentageError::OutOfRange => write!(f, "Percentage must be between 0 and 100")
        }
    }
}

impl Error for ParsePercentageError { }

pub fn parse_percentage(input: &str) -> Result<u8, ParsePercentageError> {
    // 3. Implement this function
    let p: i32 = input.parse::<i32>().map_err(|_| ParsePercentageError::InvalidInput)?;

    match p {
        0..=100 => Ok(p as u8),
        _ => Err(ParsePercentageError::OutOfRange)
    }
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    println!("{:?}", result); // Should print: Ok(50)

    let result = parse_percentage("101");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::OutOfRange)

    let result = parse_percentage("abc");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::InvalidInput)
}
