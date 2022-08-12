#[allow(unused_variables)]
fn main() {
    let my_tup: (u32, f32, i8) = (200, 3.14, -5);
    let other_tup = ("Alphabet", 5_00_000);
    let pi_val = my_tup.1;
    let (my_company, _) = other_tup;
    let (another_reference, emp_count) = other_tup;
    println!("pi: {}, company: {}, emp_count: {}", pi_val, my_company, emp_count);
}