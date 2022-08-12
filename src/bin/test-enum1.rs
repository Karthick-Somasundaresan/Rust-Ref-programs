
#![allow(unused_variables)]
#![allow(dead_code)]
use std::cmp::Ordering;
use std::fmt;

enum Suite {
    Spade,
    Clubs,
    Hearts,
    Diamonds,
    Joker,
}

impl fmt::Display for Suite {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Suite::Spade => write!(f, "S"),
            Suite::Clubs => write!(f, "C"),
            Suite::Hearts => write!(f, "H"),
            Suite::Diamonds => write!(f, "D"),
            Suite::Joker => write!(f, "Jkr"),
            
        }
    }
}

impl PartialEq for Suite {
    fn eq (&self, Other: &Self ) -> bool {
        match (self, Other) {
            (Suite::Joker, Suite::Joker) => true,
            (Suite::Joker, _) => false,
            (_, Suite::Joker) => false,
            _ => true
        }
    }
}

impl PartialOrd for Suite {
    fn partial_cmp(&self, Other: &Self) -> Option<Ordering> {
        match(self, Other) {
            (Suite::Joker, _) => Some(Ordering::Greater),
            (_, Suite::Joker) => Some(Ordering::Less),
            _ => Some(Ordering::Equal)
        }
    }
}

fn main() {
    let s = Suite::Diamonds;
    let s1 = Suite::Spade;
    // let s1 = Suite::from_str("D");
    println!("{:?} {:?}",s.to_string(), s1.to_string()); 
    println!("{:?}", s > s1);
    println!("{:?}", s == s1);
    println!("{:?}", s < s1);
}