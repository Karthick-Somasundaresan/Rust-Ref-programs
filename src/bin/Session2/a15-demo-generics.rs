// #[warn(dead_code)]
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}
fn main() {
    let p1 = Point{x:1, y:2};
    let p2 = Point {x:2.4, y:5.6};
    println!("Point in i32: {:?}", p1);
    println!("Point in f64: {:#?}", p2);
    if 1 == 1 {
        print!("asdcasd");
    } else {
        print!("asdcasd");
    }
    return;
}