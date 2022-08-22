
use std::{collections::HashMap, sync::Arc};

pub trait SomeTrait {
    fn some_trait_function(&mut self);
    fn magic_function(&self, method: String);
}
#[derive(Default)]
struct SomeStruct {
    function_map: HashMap<String, fn (&Self) -> Result<String, u8>>
}
impl SomeTrait for SomeStruct {
    fn some_trait_function(&mut self) {
        println!("Implementation of some trait function");
        self.initialize();
    }

    fn magic_function(&self, method: String) {
        if let Some(&func) = self.function_map.get(&method) {
            let res = (func)(&self);
            println!("Result: {:?}", res);

        }
    }
}
impl SomeStruct {
    fn add_handling_function(&mut self, method: String, handling_function: fn(&Self)->Result<String, u8>) {
        self.function_map.insert(method, handling_function);
    }
}
impl SomeStruct {

    fn initialize(&mut self) {
        self.add_handling_function("String1".to_owned(), SomeStruct::function1);
        self.add_handling_function("String2".to_owned(), SomeStruct::function2);
    }
    fn function1(&self) -> Result<String, u8> {
        Ok("From Function1".to_owned())
    }

    fn function2(&self) -> Result<String, u8> {
        Ok("From Function2".to_owned())
    }
}
fn main() {
    let mut ss : SomeStruct = Default::default();
    ss.some_trait_function();
    ss.magic_function("String1".to_owned());

}