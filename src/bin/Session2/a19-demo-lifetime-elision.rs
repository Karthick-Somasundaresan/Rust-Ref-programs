#![allow(dead_code)]
#[derive(Debug)]
struct Employee {
    name: String,
    id: u32
}

impl Employee {
    fn new(name: &str, id: u32) -> Employee {
        Employee { name: String::from(name), id: id }
    }
}


fn main() {
    let str1 = String::from("Short string");
    let str2 = String::from("Long Sentence");
    let long_len = longest_len(&str1, &str2);
    println!("Longest len: {}", long_len);
    println!("First word of str1: {}", first_word(str1.as_str()));
    println!("First word of str2: {}", first_word(str2.as_str()));
    let e = Employee::new("Bill", 1042523);
    println!("Employee name: {}", e.get_employee_name());

}


//Rule 1:
fn longest_len(s1: &str, s2: &str) -> usize {
    if s1.len() > s2.len() {
        s1.len()
    } else {
        s2.len()
    }
}

//Rule 2:
fn first_word(line: &str) -> &str{
    for (i,c) in line.chars().enumerate() {
        if c == ' ' {
            return &line[0..i];
        }
    }
    &line
}

//Rule 3:
impl Employee{
    fn get_employee_name(&self) -> &str {
        &self.name
    }
}