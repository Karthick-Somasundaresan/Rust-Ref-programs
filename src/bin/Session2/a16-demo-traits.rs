#![allow(dead_code)]
#![allow(unused_variables)]
use std::fmt;

pub trait Intro {
    fn introduce(&self) {
        println!("Hi");
    }
}
#[derive(Debug)]
struct Employee {
    name: String,
    id: u64,
    experience: u8
}

#[derive(Debug, Default)]
struct Student {
    name: String,
    id: u64,
    grade: u8
}

impl Employee {
    fn new(name: &str, id: u64, exp: u8) -> Employee {
        Employee { name: String::from(name), id, experience:exp }
    }
    fn employee_details(&self) -> (&String, &u64) {
        (&self.name, &self.id)
    }
}
impl Student {
    fn new(name: &str, id: u64, grade: u8) -> Student {
        Student { name: String::from(name), id, grade:grade }
    }
    fn student_details(&self) -> (&String, &u64) {
        (&self.name, &self.id)
    }
}
// Syntax for impl a trait. 
// impl <trait name> for <struct name> {}
impl Intro for Employee {
    fn introduce(&self) {
        println!("Hi! I'm {} I've {} years of experience ", self.name, self.experience);
    }
}

impl std::fmt::Display for Employee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}'s id: {}", self.name, self.id)
    }
}

fn main() {
    let e = Employee::new("Elon", 10406179, 13);
    let s:Student = Default::default();
    let s1 = Student::new("Ananya", 19013, 2);
    let (name, id) = e.employee_details();
    e.introduce();
    println!("id: {}", id);
    println!("name: {}", name);
    println!("Employee struct (Using Debug trait): {:?}", e); // {:?} =>uses Debug trait. placefolder for Display is {}
    println!("Employee struct (Using Display trait): {}", e); // placefolder for Display is {}
    println!("Student struct (Using Debug trait) {:?}", s);
    println!("Student struct (Using Display trait): {:?}", s1); // placefolder for Display is {}
}