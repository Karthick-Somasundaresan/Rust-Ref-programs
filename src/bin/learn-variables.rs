
fn main() {
    let mut x = 6;
    // x = 12;
    println!("Value of x: {}", x);
    test_function(&mut x);
    println!("Value of x: {}", x);
}

fn test_function(var: &mut i32) {
    *var = 23;
    let mut local_x = 223;
    let var = &mut local_x;
    println!("var: {}", var);
}