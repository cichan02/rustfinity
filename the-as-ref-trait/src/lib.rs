pub fn print_message<T> (x: T) // Finish the function
where
    T: AsRef<str>
{
    println!("{}", x.as_ref());
}

// Example usage
pub fn main() {
    // Example 1: Using a &str
    print_message("Hello, world!");

    // Example 2: Using a String
    let greeting = String::from("Welcome to Rust!");
    print_message(greeting);
}
