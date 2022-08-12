fn main() {
    let x = 3.0;
    let y = 1.0;
    let result = {
        if y != 0.0 {
            Some(x/y)
        } else {
            None
        }
    };
    match result {
        Some(val) => {println!("Result: {}", val);},
        None => {println!("Result: Cant divide by zero" );}
    }
}