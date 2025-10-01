#[derive(Debug)]
struct Student {
    name: String,
    major: String,
}

impl Student {
    fn new(name: &str, major: &str) -> Self {
        Self {
            name: name.to_string(),
            major: major.to_string(),
        }
    }

    fn set_major(&mut self, major: &str) {
        self.major = major.to_string();
    }

    fn get_major(&self) -> &str {
        &self.major
    }
}

fn main() {    
    let mut s = Student::new("Emily Gomez", "Business");
    println!("Student created: {:?}, major: {}", s.name, s.get_major());

    s.set_major("Computer Science");
    println!("After change: {} is now majoring in {}", s.name, s.get_major());
}
