
fn check_life_time_1() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The Longest String: {}", result);
}
// This function check_life_time_2() compilation is failing as longest() function can return y which lifetime is finish in internal block "{}"
//
fn check_life_time_2() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The Longest String: {}", result);
}
// &i32  // a reference
// &'a i32 // a reference with explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime
// Life time of the return value should be smallest life time of the parameters. x and y has different lifetime.
// 
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y   
    }
}

fn main() {
    check_life_time_1();
}