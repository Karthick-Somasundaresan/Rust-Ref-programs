// use std::convert::From;

// mod FirstMod {
// pub struct OneType {
//     mem_var_type: String
//  }

// impl OneType {

//     pub fn new() -> Self {
//         OneType { mem_var_type: "SomeType".to_owned() }
//     }
//     pub fn some_function(&self) {
//         println!("Some implementation of {}", self.mem_var_type );
//     }
// }



// }

// mod SecondMod {
//     impl crate::FirstMod::OneType {
//         fn some_function(&self) {
//             println!("Overriding some_function impl")
//         }
//     }
// }
// fn test_function(ot: Box<crate::FirstMod::OneType>) {
//     ot.some_function();
// }

// fn main() {
//     use crate::FirstMod::OneType;
//     println!("Main function");
//     test_function(Box::new(OneType::new()));

// }

fn main() {}