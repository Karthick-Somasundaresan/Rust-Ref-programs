pub trait SomeTrait {
    fn some_trait_function(&self);
}
pub struct SomeStruct {
    member: u8
}

impl SomeStruct {
    fn some_memeber_method(&self) {
        println!("some_member_method implementation: {}", self.member);
    }
}

impl SomeTrait for SomeStruct {
    fn some_trait_function(&self) {
        self.some_memeber_method();
    }
}

fn main() {
    let some_var = SomeStruct{member:4};
    some_var.some_trait_function();
}