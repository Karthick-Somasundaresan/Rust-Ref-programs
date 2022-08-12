
const v: i32 = 132;
fn add_two(x: i32) -> i32 {
    x + 2 // notice that we dont have a semi-colon
    // return x + 2; // notice that we dont have a semi-colon
}

fn make_even(x:i32) -> i32 {
    if x % 2 == 0 {
        x // no semi-colon here as well.
    } else {
        x + 1 // no semi-colon here as well.
    }
}

fn main() {
    let y = add_two(5);
    println!("Value of y: {}", y);
    println!("Even Value: {}", make_even(y));
}