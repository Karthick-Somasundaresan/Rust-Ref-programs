
use std::{collections::HashMap, sync::Arc};

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
    fn get_function_map() -> HashMap<String, (Arc<Box<SomeStruct>>, fn(&Self)->Result<String, u8>)> {
        let mut map:HashMap<String, (Arc<Box<SomeStruct>>, fn(&Self)->Result<String, u8>)> = HashMap::new();
        let arc_obj = Arc::new(Box::new(SomeStruct{}));
        map.insert("String1".to_owned(), (arc_obj.clone(), SomeStruct::function1));
        map.insert("String2".to_owned(), (arc_obj.clone(), SomeStruct::function2));
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
    let my_func = fun_map.get("String2");
    if let Some(func) = my_func {
        let (obj, meth) = func;
        let result = meth(obj);
        println!("Result: {:?}", result);

    }
}