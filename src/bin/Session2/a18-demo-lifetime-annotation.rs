fn main () {
    let s1 = String::from("one"); //'b
    let long_str;
    {
        let s2 = String::from("five"); //'a 
        long_str = longest(s1.as_str(), s2.as_str()); //'a
        println!("Longest String: {}", long_str);
    }

}

fn longest<'r>(s1:&'r str, s2:&'r str) -> &'r str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}