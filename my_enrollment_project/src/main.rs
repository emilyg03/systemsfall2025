use std::fmt::Display;

pub trait ShowInfo {
    fn show_info(&self) -> String;
}

impl ShowInfo for Box<dyn ShowInfo> {
    fn show_info(&self) -> String {
        // Delegate to the inner object
        (**self).show_info()
    }
}

// Shared student base 
pub struct StudentBase {
    pub gpa: f32,
    pub major: String,
}

impl StudentBase {
    pub fn new(gpa: f32, major: impl Into<String>) -> Self {
        Self {
            gpa,
            major: major.into(),
        }
    }
    pub fn base_info(&self) -> String {
        format!("major: {}, gpa: {:.2}", self.major, self.gpa)
    }
}

// Undergraduate
pub struct Undergrad<ID> {
    pub id: ID,
    pub year: u8,
    pub base: StudentBase,
}

impl<ID: Display> Undergrad<ID> {
    pub fn new(id: ID, year: u8, base: StudentBase) -> Self {
        Self { id, year, base }
    }
}

impl<ID: Display> ShowInfo for Undergrad<ID> {
    fn show_info(&self) -> String {
        format!(
            "Undergrad (id: {}) year: {} | {}",
            self.id,
            self.year,
            self.base.base_info()
        )
    }
}

// Graduate
pub struct Grad<ID> {
    pub id: ID,
    pub base: StudentBase,
    pub thesis: String,
}

impl<ID: Display> Grad<ID> {
    pub fn new(id: ID, thesis: impl Into<String>, base: StudentBase) -> Self {
        Self {
            id,
            thesis: thesis.into(),
            base,
        }
    }
}

impl<ID: Display> ShowInfo for Grad<ID> {
    fn show_info(&self) -> String {
        format!(
            "Grad (id: {}) thesis: '{}' | {}",
            self.id,
            self.thesis,
            self.base.base_info()
        )
    }
}

// Enrollment
pub struct Enrollment<T: ShowInfo> {
    students: Vec<T>,
}

impl<T: ShowInfo> Enrollment<T> {
    pub fn new() -> Self {
        Self { students: Vec::new() }
    }

    pub fn add(&mut self, s: T) {
        self.students.push(s);
    }

    pub fn all_infos(&self) -> Vec<String> {
        self.students.iter().map(|s| s.show_info()).collect()
    }

    pub fn print_all(&self) {
        for info in self.all_infos() {
            println!("{}", info);
        }
    }
}

fn main() {
    let ug_base = StudentBase::new(3.6, "Computer Science");
    let grad_base = StudentBase::new(3.9, "Computer Science");

    let u1 = Undergrad::new("u123", 2, ug_base);
    let g1 = Grad::new(42u32, "Type-level programming in Rust", grad_base);

    let mut roster: Enrollment<Box<dyn ShowInfo>> = Enrollment::new();
    roster.add(Box::new(u1));
    roster.add(Box::new(g1));

    roster.print_all();
}
