
fn main() {
    let  x = 5;
    println!("Value of x is {}", x);
    let x = "Change to String => Called Shadowing";
    println!("Value of x is {}", x);

    //const
    const SUBSCRIBER_COUNT: u32 = 100_000;
    println!("Value of const Value: {}", SUBSCRIBER_COUNT);
    
    // Tuple
    let tup = ("Hrudaya", 100);
    println!("Tuple Values => {}, {}", tup.0, tup.1);
    let copy_tup = tup;
    println!("Copy Tuple => {}, {}", copy_tup.0, copy_tup.1); 

    // Array (It has a fix size, if need dynamic size, then need to use Vector)
    let error_codes = [200, 404, 500];
    println!("Value of ErrorCodes {}, {}, {}", error_codes[0], error_codes[1], error_codes[2]);
    // Set all 3 fileds of the array zero
    let byte = [0; 3];
    println!("Value of Array byte: {} {} {}", byte[0], byte[1], byte[2]);
    println!("Sum of the Numbers are: {}", my_function(4, 4));

    // control flow
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("Value of Number: {}", number);

    // Different Kind of Loop
    // 1
    loop  { 
        println!("loop!!!");
        break;
    }
    // 2
    let mut number = 2;
    while number > 0 {
        println!("Inside While");
        number = number - 1;
    }
    // 3
    let a = [10, 20, 30, 40, 50];
    for ele  in a {
        println!("Values: {}", ele);
    }
    for numb in 1..4 {
        println!("{}!", numb);
    }


}
fn my_function(x: i32, y: i32) -> i32{
    println!("Value of x: {},  y: {}", x, y);
    // last statement of the function is consider as return statement. No need to put ; in return statement
    x + y
}
  