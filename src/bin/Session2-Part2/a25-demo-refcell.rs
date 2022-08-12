use std::cell::RefCell;


#[derive(Debug)]
struct User {
    name: String,
    id: u32,
    active: RefCell<bool>
}

fn main() {
    let user1 = User {name: "User1".to_owned(), id: 12313, active: RefCell::new(false)};
    {
        let mut status = user1.active.borrow_mut();
        // Uncommenting the below code will not throw a compilation error but will panic at runtime
        // let mut status1 = user1.active.borrow_mut();
        *status = true;
        // *status1 = true;
    }
    // drop(status);
    *user1.active.borrow_mut() = false;
    *user1.active.borrow_mut() = true;
    println!("User1 details: {:?}", user1);

}

// #[derive(Debug)]
// struct User {
//     name: String,
//     id: u32,
//     active: bool
// }

// fn main() {
//     let user1 = User {name: "User1".to_owned(), id: 12313, active: false};
//     {
//         let mut status = user1.active;
//         status = true;
//     }
//     println!("User1 details: {:?}", user1);

// }