use std::fmt;
//We can also have structs associated with the variants
#[derive(Debug)]
enum Action {
    Move { x: i32, y: i32 },
    ChangeColor(i32, i32, i32),
    Rotate{deg: u16},
    Zoom(u8)
}

fn perform(cmd: Action) {
    println!("Received Command: {} ", cmd);
    //match - simialr to switch, helps to extract data associated with the variant.
    match cmd {
        Action::Move { x, y } if x==0 && y ==0 => {println!("Moving to origin");}
        Action::Move {x, y} => {println!("Moving to location: {}, {}", x, y);}
        Action::ChangeColor(r, g, b) => {println!("Changing to Color ({}, {}, {})", r ,g, b);}
        _ => {println!("Command not yet implemented")}
    }
}

//Enum having a function implementation.
impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Action::ChangeColor(r ,g ,b ) => write!(f, "Chaning color to {} {} {}", r,g,b),
            Action::Move{x ,y } => write!(f, "Moving to {} {}", x, y),
            Action::Rotate{deg} => write!(f, "Rotate {} degree", deg),
            Action::Zoom(val ) => write!(f, "Zoom {}%", val),
        }
   }
}
fn main() {

    perform(Action::Move { x: 0, y: 0 });
    perform(Action::Move { x: 100, y: 200 });
    perform(Action::ChangeColor(220, 220, 220));
    perform(Action::Rotate { deg: 270 });
    perform(Action::Zoom(150));

}
