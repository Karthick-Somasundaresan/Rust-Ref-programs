#![allow(unused_variables)]
fn main() {
    let mut v = Vec::new();
    v.push(12);
    v.push(13);
    v.push(14);
    let v1 = vec![1, 3, 5 , 7];

    for i in &mut v {
        *i += 10;
    }

    for i in v {
        print!("{}, ", i);
    }
    println!();
}