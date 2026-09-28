use std::collections::HashMap;

pub struct Student {
    pub name: String,
    pub grades: Vec<u8>,
}

impl Student {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            grades: Vec::new()
        }
    }

    pub fn add_grade(&mut self, grade: u8) {
        // Implement here
        self.grades.push(grade);
    }

    pub fn average_grade(&self) -> f64 {
        // Implement here
        let len: usize = self.grades.len();
        if len == 0 {
            return 0.0;
        }

        let mut sum: usize = 0;
        for grade in &self.grades {
            sum += *grade as usize;
        }
        return sum as f64 / len as f64;
    }
}

pub struct StudentGrades {
    pub students: HashMap<String, Student>,
}

impl StudentGrades {
    pub fn new() -> Self {
        Self {
            students: HashMap::new(),
        }
    }

    pub fn add_student(&mut self, name: &str) {
        self.students
            .entry(name.to_string())
            .or_insert(Student::new(name));
    }

    pub fn add_grade(&mut self, name: &str, grade: u8) {
        if let Some(student) = self.students.get_mut(name) {
            student.add_grade(grade);
        }
    }

    pub fn get_grades(&self, name: &str) -> &[u8] {
        self.students
            .get(name)
            .map(|s| s.grades.as_slice())
            .unwrap_or(&[])
    }
}

pub fn main() {
    let mut tracker = StudentGrades::new();

    tracker.add_student("Alice");
    tracker.add_student("Bob");

    tracker.add_grade("Alice", 85);
    tracker.add_grade("Alice", 90);
    tracker.add_grade("Bob", 78);

    let alice = tracker.students.get_mut("Alice").unwrap();

    alice.add_grade(95);
    println!("{:?}", alice.grades);
    println!("{:?}", alice.average_grade());
    println!("{:?}", tracker.get_grades("Bob"));
}
