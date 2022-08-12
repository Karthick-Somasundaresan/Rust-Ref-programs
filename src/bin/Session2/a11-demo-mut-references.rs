fn main() {
    {
        let mut owner = String::from("Rust Lang");
        println!("Length of the string: {}",get_length(&owner));
        let mut_ref1 = &mut owner; 
        mut_ref1.push_str("age.");
        println!("value of string: {}", owner);
        let mut_ref2 = &mut owner; 
        mut_ref2.push_str(" is new to us");
        /* Uncommenting the below line would throw error 
         * second mutable borrow
         */
        // println!("{}", mut_ref1);
        let r1 = &owner;
        let r2 = &owner;
        println!("String using r1: {}", r1);
        println!("String using r2: {}", r2);
    }
}

fn get_length(ref_var: &String) -> usize {
    ref_var.len()
}
