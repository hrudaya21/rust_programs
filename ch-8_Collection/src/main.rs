use std::collections::HashMap;
fn main() {
    // check_for_vector();
    // check_for_string();
    check_for_hashmap();
}
fn check_for_hashmap() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();
    scores.insert(blue, 10);
    scores.insert(yellow, 20);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    // If entry already present don't insert, else insert 30
    scores.entry(String::from("Orange")).or_insert(30);

    // Track the word Frequency usng Map
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);
}
fn check_for_string() {
    let s1 = String::new();
    let s2 = "S2: Initial Content";
    let s3 = s2.to_string();
    let s4 = String::from("S4: Initial Content");

    println!("{}, {}, {}, {}", s1, s2, s3, s4);

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    println!("Current Value: {}", s);

    let s1 = String::from("Hello");
    let s2 = String::from("World!!");
    //let s3 = s1 + &s2;
    let s3 = format!("{}{}", s1, s2);
    println!("ConcatenateValue: {}, {}, {}", s3, s1, s2);

    for i in s3.chars() {
        println!("Each CHar: {}", i);
    }
}
fn check_for_vector() {
    let a = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    
    let mut v2 = vec![1, 2, 3];
    let third = v2[2];
    v2.push(6);
    println!("The third Element: {}", third);
    println!("Value of the Vector: {:?}", v2);

    // More Safer Way to avoid out of Index issue
    match v2.get(2) {
        Some(value) => println!("Value of xth Index: {}", value),
        None => println!("Index Value not Found!!!")
    }

    // Iterating through Vector
    for i in &v2 {
        println!("{}", i);
    }
    for i in &mut v2 {
        *i += 50;
    }
    println!("Value Vector: {:?}", v2);
}
