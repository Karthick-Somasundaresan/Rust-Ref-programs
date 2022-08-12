fn main() {
    let r;
        let x = 33;
    {
        r = &x;
        println!("{}",x);
    }
    println!("{}",r);
}