use std::collections::HashMap;

pub struct Student {
    // 1. Define the fields
    pub name: String,
    pub grades: Vec<u8>
}

impl Student {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            grades: Vec::new()
        }
    }
}

pub struct StudentGrades {
    pub students: HashMap<String, Student>
}

impl StudentGrades {
    pub fn new() -> Self {
        Self {
            students: HashMap::new(),
        }
    }

    // 3. Implement the methods
    pub fn add_student(&mut self, name: &str) {
        self.students
            .entry(name.to_string())
            .or_insert(Student::new(name));
    }

    pub fn add_grade(&mut self, name: &str, grade: u8) {
        if let Some(student) = self.students.get_mut(name) {
            student.grades.push(grade);
        }
    }

    pub fn get_grades(&self, name: &str) -> &[u8] {
        self.students
            .get(name)
            .map(|s| s.grades.as_slice())
            .unwrap_or(&[])
    }
}

// Example usage
pub fn main() {
    let mut tracker = StudentGrades::new();

    tracker.add_student("Alice");
    tracker.add_student("Bob");

    tracker.add_grade("Alice", 85);
    tracker.add_grade("Alice", 90);
    tracker.add_grade("Bob", 78);

    println!("{:?}", tracker.get_grades("Alice")); // [85, 90]
    println!("{:?}", tracker.get_grades("Bob")); // [78]
}
