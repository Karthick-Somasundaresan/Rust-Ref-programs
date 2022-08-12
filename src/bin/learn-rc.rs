use std::{rc::Rc, borrow::BorrowMut};

fn main() {
    let my_str = Rc::new(String::from("String in Heap"));
    let oth_str = Rc::clone(&my_str);
    
}

// fn string_modify(mut input: Rc<String>) {
//     let mut oth_str = &*input.borrow_mut();
//     oth_str.push_str("!");

// }