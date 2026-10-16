use std::{
    fs::File,
    io::{
        BufRead, BufReader, Error, ErrorKind
    },
    num::ParseIntError
};

pub fn sum_integers_from_file(file_path: &str) -> Result<i32, Error> {
    // TODO: Implement this function
    // Hint: Use `File::open`, `BufReader::new`, and `.lines()` to process the file.
    // Use `?` to propagate errors and `io::Error::new` for custom errors.
    let f = File::open(file_path)?;
    BufReader::new(f)
        .lines()
        .map(|l| {
            l?
                .parse::<i32>()
                .map_err(|r: ParseIntError| Error::new(ErrorKind::InvalidData, r))
        })
        .sum()
}

// Example usage
pub fn main() {
    let file_path = "numbers.txt";

    match sum_integers_from_file(file_path) {
        Ok(sum) => println!("The sum is: {}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}
