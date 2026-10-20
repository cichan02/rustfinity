// Finish the function
pub fn filter_starts_with<'a, 'b> (col: &'a [String], search: &'b str) -> impl Iterator<Item = &'a String> + use<'a, 'b>{
    col
        .iter()
        .filter(move |s| s.starts_with(search))
}

// Example usage
pub fn main() {
    let input = vec![
        String::from("apple"),
        String::from("apricot"),
        String::from("banana"),
        String::from("cherry"),
    ];
    let filtered: Vec<&String> = filter_starts_with(&input, "ap").collect();
    println!("{:?}", filtered); // Expected output: ["apple", "apricot"]
}
