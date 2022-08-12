fn main () {
    let s1 = String::from("one"); //'b
    let long_str;
    {
        let _s2 = String::from("five"); //'a 
        // long_str = longest(s1.as_str(), _s2.as_str()); //'a
        long_str = longest(&s1);
    }
    println!("Longest String: {}", long_str);

}

// fn longest<'r>(s1:&'r str, s2:&str) -> &'r str {
fn longest(s1:& str) -> &str {
    // if s1.len() > s2.len() {
    //     s1
    // } else {
    //     s2
    // }
    s1
}