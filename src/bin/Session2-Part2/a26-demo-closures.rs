
fn main() {
    let mut v:Vec<i32> = (1..=100).collect();
    /*
     * This borrows v as reference. The variable 
     * is still accessible in main.
     */
    let sze = || { println!("Size of vec: {}", v.len());}; 
    sze();
    println!("First element: {}", v[0]);
    /*
     * This borrows v mutable reference. The variable
     * is still accessible in main.
     */
    let mut add_elem = || v.push(101);
    add_elem();
    println!("Size of vector: {}", v.len());

    /*
     * This moves the variable v into the closure
     * The variable is not accessible in main.
     */
    {
        let mut own = move || -> Vec<i32> {v.pop(); println!("Size of vector: {}", v.len()); v };
    v = own();
    }
    println!("Size of vector: {}", v.len());
}