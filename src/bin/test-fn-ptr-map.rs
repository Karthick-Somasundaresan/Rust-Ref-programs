
use std::collections::HashMap;

pub trait SomeTrait {
    fn some_trait_function(&self);
}
struct SomeStruct {
    // fn get_function_map() -> HashMap<String, fn()->Result<String, u8>>;
}
impl SomeTrait for SomeStruct {
    fn some_trait_function(&self) {
        println!("Implementation of some trait function");
    }
}
impl SomeStruct {
    fn get_function_map() -> HashMap<String, (Self, fn(&Self)->Result<String, u8>)> {
        let mut map:HashMap<String, (Self, fn(&Self)->Result<String, u8>)> = HashMap::new();

        map.insert("String1".to_owned(), (Self{}, SomeStruct::function1));
        map.insert("String2".to_owned(), (Self{}, SomeStruct::function2));
        map
    }

    fn function1(&self) -> Result<String, u8> {
        Ok("From Function1".to_owned())
    }

    fn function2(&self) -> Result<String, u8> {
        Ok("From Function2".to_owned())
    }
}
fn main() {
    // let ss = SomeStruct{};
    let fun_map = SomeStruct::get_function_map();
    let my_func = fun_map.get("String1");
    if let Some(func) = my_func {
        let (obj, meth) = func;
        let result = meth(obj);
        println!("Result: {:?}", result);

    }
}