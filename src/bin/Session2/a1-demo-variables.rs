/*
 * Variables can be shadowed. The shadowed variable can be of different type.
 */
fn main() {
    let x = 32;
    // let x: i32 = 34;
    {
        let x= 33;
        println!("x inside block = {}",x);
        let y = x + 2;
        println!("y inside block = {}", y);
    }
    println!("x outside block = {}",x);
    let x = "SomeString"; //Shadow
    println!("Shadowed x = {}",x);

}