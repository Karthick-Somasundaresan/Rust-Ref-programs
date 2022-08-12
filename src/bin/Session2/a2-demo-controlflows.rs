fn main () {
    let a = -1;
    let b = [12, 32, 53, 71];
    if a < 0 {
        println!(" {} is a negative number!", a);
    } else {
        println!(" {} is a positive number!", a);
    }
    println!("Numbers in the array:");
    for nums in b {
        println!("Range for loop :{} ", nums * 2);
    }
    let mut count = 20;
    loop {
        println!("in loop {}", count);
        count -= 1;
        if count < 10 {
            break;
        }
    }
    
    while count > 0 {
        println!("in while {}", count);
        count -= 1;
    }
    
}