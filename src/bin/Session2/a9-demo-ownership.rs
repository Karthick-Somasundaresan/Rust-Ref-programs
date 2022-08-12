fn main() {
    let owner = String::from("hello");
    some_function(owner); // Ownership is passed to the function
                                    // ... and so is no longer valid here
    // println!("{}", owner); // This results in an error. 

    let x = 5;
    makes_copy(x); //Only a copy is passed to the function 
                                //so it is okay to use afterwards

    println!("{}", x); // This works because the value is in the stack and not heap.

} 

fn some_function(new_owner: String) { // some_string comes into scope
    println!("{}", new_owner);
} // Here, new_owner goes out of scope and `drop` is called and memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.