struct Employee {
    name: String,
    id: u64
}
impl Employee {
    fn new(name: &str, id: u64) -> Employee {
        Employee { name: String::from(name), id }
    }
    fn employee_details(&self) -> (&String, &u64) {
        (&self.name, &self.id)
    }
}

fn main() {
    let e = Employee::new("Karthick", 10406179);
    let (name, id) = e.employee_details();
    println!("id: {}", id);
    println!("name: {}", name);
}