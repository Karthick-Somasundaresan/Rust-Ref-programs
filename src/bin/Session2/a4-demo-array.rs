fn main() {
    let arr = [0;6];
    let arr2 = arr;
    println!("arr = {:?}",arr);
    let arr = [1, 3, 5, 7, 9, 11];
    let mid = &arr[1..=4];
    println!("Shadowed arr = {:?}", arr);
    println!("arr2 = {:?}", arr2);
    println!("mid = {:?}", mid);
}