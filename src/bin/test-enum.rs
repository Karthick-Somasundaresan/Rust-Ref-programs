use std::cmp::Ordering;

pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}


fn main() {
    array_funcs();
}

fn array_funcs() {
    let a = [1,2,2];
    let b = [1,1,1,2];
    match compare(&a, &b) {
        Comparison::Equal => {println!("Arrays a={:?} and b={:?} are same!", &a, &b);},
        Comparison::Unequal => {println!("Arrays a={:?} and b={:?} are Different!", a, &b);},
        Comparison::Sublist => {println!("Array a={:?} is subset of b={:?}!", &a, &b);},
        Comparison::Superlist => {println!("Array a={:?} is superset of b={:?}!", &a, &b);},
        
    }
}


pub fn compare<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match _first_list.eq(_second_list) {
        true => Comparison::Equal,
        false => match _first_list.len().cmp(&_second_list.len()) {
            Ordering::Equal => {println!("Returning Unequal!!");Comparison::Unequal},
            Ordering::Less => {println!("Check for Sublist"); check_sublist(_first_list, _second_list)}
            Ordering::Greater => {println!("Check for Superlist!"); check_superlist(_first_list, _second_list)}
        }
    }
}

fn check_superlist<T:PartialEq>(_first_list: &[T], _second_list:&[T]) -> Comparison {
    assert!(_first_list.len().gt(&_second_list.len()));
    if _second_list.len() == 0 {
        Comparison::Superlist
    } else {
        match contains_all(_second_list, _first_list) {
            true => Comparison::Superlist,
            false => Comparison::Unequal
        }    
    }
}

fn check_sublist<T:PartialEq>(_first_list: &[T], _second_list:&[T]) -> Comparison {
    assert!(_first_list.len().lt(&_second_list.len()));
    if _first_list.len() == 0 {
        Comparison::Sublist
    } else {
        match contains_all(_first_list, _second_list) {
            true => Comparison::Sublist,
            false => Comparison::Unequal
        }
    }
    
}

fn contains_all <T:PartialEq>( _first_list: &[T], _second_list: &[T]) -> bool {
    assert!(_first_list.len().lt(&_second_list.len()));
    let mut all_present = false;
    let mut index = 0;
    while _second_list.len() - index >= _first_list.len() {
        let second_start_index = index;
        let mut iter = 0;
        let mut not_match = false;
        while iter < _first_list.len() {
            if _first_list[iter] != _second_list[second_start_index + iter] {
                not_match = true;
                break;
            }
            iter +=1;
        }
        if not_match {
            index+=1;
        } else {
            all_present = true;
            break;    
        }
        
    }
    all_present
}


