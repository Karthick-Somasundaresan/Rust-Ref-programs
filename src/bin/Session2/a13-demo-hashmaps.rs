fn main() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //Ownership is with the hash map
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    // let f1 = &field_name;
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // Uncommenting the below code would throw an error
    // println!("key: {} value: ", f1);

    //Accessing Values in a Hash Map
    if let Some(score) = scores.get("Green") {
        println!("Green team score: {}", score);
    } else {
        println!("Green team yet to score!");
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //Updating a value.
    scores.insert(String::from("Blue"), 60);
    println!("After Blue update {:?}", scores);

    //Conditional update
    scores.entry(String::from("Green")).or_insert(0);
    println!("After Green insert {:?}", scores);
}