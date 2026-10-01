pub fn parse_percentage(input: &str) -> Result<u8, String> {
    // TODO: Implement the function here
    let p: i32 = input.parse::<i32>().map_err(|_| "Invalid input".to_string())?;

    match p {
        0..=100 => Ok(p as u8),
        _ => Err("Percentage out of range".to_string())
    }
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    assert_eq!(result, Ok(50));

    let result = parse_percentage("101");
    assert_eq!(result, Err("Percentage out of range".to_string()));

    let result = parse_percentage("abc");
    assert_eq!(result, Err("Invalid input".to_string()));
}
