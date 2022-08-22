use std::collections::HashMap;

pub struct SomeStruct;

impl SomeStruct {
    fn get_function_map(&self) -> HashMap<String, Box<dyn Fn()->Result<String, u8>>>{

        let mut function_map = HashMap::new();
        function_map.insert("String1".to_owned(), Box::new(|| -> Result<String, u8>{self.function1()}));
        function_map
    }
    fn function1(&self) -> Result<String, u8> {
        Ok("From Function1".to_owned())
    }
    fn function2(&self) -> Result<String, u8> {
        Ok("From Function2".to_owned())
    }
}

fn main() {

}