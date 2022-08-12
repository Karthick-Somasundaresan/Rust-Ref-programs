fn main() {
    let owner = String::from("Rust Lang");
    println!("Length of the string: {}",get_length(&owner));
    println!("value of string: {}", owner);
}

fn get_length(ref_borrow: &String) -> usize {
    ref_borrow.len()
}