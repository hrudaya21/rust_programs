fn main() {
    // 1
    let s1 = String::from("Hello"); // This store the String in the Heap;
    let s2 = s1.clone(); // Need to clone if wanted to print s1 in next line
    println!("Value of s1: {}, s2: {}", s1, s2);

    // 2
    // Integer doesn't need the same borrowing as String
    let x = 10;
    let y = x;
    println!("Value of x: {}, y: {}", x, y);

    // 3
    let s3 = String::from("Hrudaya");
    let len = get_length_string(&s3);
    //To Print s3, in following line, need to pass the reference of s3 in the function which can't be changed inside the function.
    // ***  Reference are Immutable in nature and Can't take the Ownership
    println!("len: {}, string: {}", len, s3); 
    // 4
    // To change the reference string inside the function, need to pass mutable reference
    let mut s4 = String::from("Hrudaya");
    let len = get_change_length_string(&mut s4); // Mutable Reference to Pass if wanted to pass the ownership
    println!("String: {}, Len: {}", s4, len);

    // 5
    // One mutable reference can be present for a single string
    let mut s5 = String::from("Helllo");
    let r1 = &mut s5;
    // let r2 = &mut s5; // 2nd Mutable reference not allowed
    // Multiple immutable reference is possible
    let r2 = &s4;
    let r3 = &s4;
    println!("{} {}", r1, r2);

    // 6 String Slice
    let input_str = String::from("Hello World!!");
    println!("4th to 6th Char: {}", &input_str[4..7]);
    println!("First 4th Char to end: {}", &input_str[3..]);

    // 7 array slice
    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];
    println!("Slice: {:?}", slice);


}

fn get_change_length_string(some_str: &mut String) -> usize {
    some_str.push_str(" Ranjan"); 
    let len = some_str.len();
    len
}
fn get_length_string(some_str: &String) -> usize {
    let len = some_str.len();
    len
}


/*
    let x 
*/