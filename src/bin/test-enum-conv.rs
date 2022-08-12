#![allow(dead_code)]
#![allow(unused_variables)]
use std::str::FromStr;
// #[derive(PartialEq)]
#[derive(Debug)]
enum Rank {
    Num(u8),
    Alpha(char),
    Non,
}

// #[derive(FromStr)]
enum Suite {
    Spade,
    Clubs,
    Hearts,
    Diamonds,
    Joker,
}

impl PartialEq for Rank {
    fn eq(&self, other: &Rank) -> bool {
        println!("");
        match self {
            Rank::Num(v) => { match other {
                Rank::Num(v1) if v1== v => {return true;},
                _ => {return false;},
            }},
            Rank::Alpha(c) => { match other {
                Rank::Alpha(c1) if c1== c => {return true;},
                _ => {return false;}
            }},
            Rank::Non => {
                            match other {
                                Rank::Alpha(_) | Rank::Num(..) => {return false;},
                                _ => {return true;}
                            }

            },
        }
    }
}

impl FromStr for Rank {
    type Err = &'static str;

    fn from_str(rnk: &str) -> Result<Self, Self::Err> {
        match rnk {
            "J" => Ok(Rank::Alpha('J')),
            "K" => Ok(Rank::Alpha('K')),
            "Q" => Ok(Rank::Alpha('Q')),
            "A" => Ok(Rank::Alpha('A')),
            "Jkr" => Ok(Rank::Non),
             _ => {
                    if let Ok(n) = rnk.parse::<u8>() {
                        match n {
                            2..=8 | 10 => Ok(Rank::Num(n)),
                            _ =>Err("Not able to convert")
                            
                        }
                    } else {
                        Err("Not able to convert!!!")
                    }
                }
        }
    }
}

fn main() {
    let r = Rank::from_str("Jkr").unwrap();
    let r1 = Rank::from_str("Jkr").unwrap();
    match r.eq(&r1) {
        true => {println!("They are same!!");},
        false => { println!("They are not the same!!"); }
    }
}

