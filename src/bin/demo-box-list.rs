#[derive(Debug)]
enum List {
    // This would throw an error since compiler will not be able to determine the size.
    // Data(u16, List),
    Data(u16, Box<List>),
    Nil
}


use crate::List::{Data, Nil};

fn main() {
    let my_list = Data(1, Box::new(Data(2, Box::new(Data(3, Box::new(Nil))))));
    println!("{:?}", my_list);

}