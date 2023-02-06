use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Generate Number: {}", secret_number);

    loop {
        println!("Please input your guess: ");
        let mut myguess = String::new();
        io::stdin()
            .read_line(&mut myguess)
            .expect("ReadLine is failed!!!");
        let myguess: u32 = match myguess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You Guessed {}", myguess);

        match myguess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small!!!".red()),
            Ordering::Greater => println!("{}", "Too Big!!!".red()),
            Ordering::Equal => {
                println!("{}", "You Win!!".green());
                break;
            },
        }
    }
    
}
