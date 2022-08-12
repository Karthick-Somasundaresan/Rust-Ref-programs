use std::vec;


fn main() {
    let mut my_vec = vec![1, 3, 5, 7];
    let mut my_sec_vec = vec![1, 3, 5, 7];
    let mut another_var = &mut my_vec;
    another_var.push(9);
    another_var = &mut my_sec_vec;
    println!("MyVec: {:?}", my_vec);
    my_vec.push(11);
    println!("MyVec: {:?}", my_vec);
    // let mut my_var = String::from("SampleString");
    // my_var.push_str("string2");

}