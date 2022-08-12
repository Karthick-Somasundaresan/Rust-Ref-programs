fn main() {
    let mut a = 40;
    let b = &mut a; //b has mutably borrowed from 'a'
    *b +=2;

    /*
     * This throws an error because compiler sees b has
     * borrowed from 'a' and still in use because of the
     * print in line 13. So accessing 'a' now would lead
     * to data race condition hence compiler throws an error 
     */
    // println!("{}", a);
    println!("{}", b);
    /*
     * This doesnt throw an error because compiler sees b is
     * not used anymore and drops it
     */
    println!("{}", a); 
}