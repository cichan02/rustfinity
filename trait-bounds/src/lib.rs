use std::{cmp::Ordering, fmt::Display};

// TODO: Define the generic function `compare_and_display` with appropriate trait bounds.
pub fn compare_and_display<T> (x: T, y: T) -> T // Complete the function definition
where
     T: Display + PartialOrd
{
     match x.partial_cmp(&y).expect("Cann't compare values") {
          Ordering::Greater => x,
          Ordering::Equal => x,
          Ordering::Less => y
     }
}

// Example usage
pub fn main() {
     let greater = compare_and_display(10, 20);
     println!("Greater value: {}", greater);

     let greater = compare_and_display("Apple", "Orange");
     println!("Greater value: {}", greater);
}
