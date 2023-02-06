use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::error::Error;
// Main Function Can have specific Error Return as shown below.
fn main() -> Result<(), Box<dyn Error>> {
    // panic!("Crash and Burn!!!"); //RUST_BACKTRACE=full cargo run <> => This command will give the complete bt.
    exampe_of_result();
    // Other Way to Write the same as Result
    example_of_result_with_closure();
    // unwrap/expect
    example_of_unwrap_expect();
    // Return Result From Function
    // read_username_from_file();
    read_username_from_file_optimized();
    Ok(())
}

fn read_username_from_file_optimized() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
fn example_of_unwrap_expect() {
    let _f1 = File::open("hello.txt").unwrap(); // This is normal panic
    let _f2 = File::open("hello.txt").expect("Fail to open hello.txt"); // This is panic with User Specific Error Code
}
fn example_of_result_with_closure() {
    let _f = File::open("hello.txt").unwrap_or_else( |error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem Creating File: {:?}", error);
            })
        } else {
            panic!("Problem Opening File: {:?}", error);
        }
    });
}
fn exampe_of_result() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        // Err(error) => panic!("Problem Opening File: {:?}", error),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem Creating File!!! {:?}", e),
            },
            other_errr => {
                panic!("Problem Opening File!! {:?}", other_errr);
            },
        }
    };
}