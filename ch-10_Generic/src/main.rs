

fn main() {
    check_generic_basic();

}
fn check_generic_basic() {
    let int_vec = vec![80, 10, 100, 90, 20, 50];
    let largest = find_greater_number(int_vec);
    println!("Int Greatest Number: {:?}", largest);
    let char_vec = vec!['a', 'z', 'c', 'd', 'e'];
    let largest = find_greater_number(char_vec);
    println!("Char Greatest Number: {:?}", largest);
}

fn find_greater_number<T: PartialOrd + Copy>(input_vec: Vec<T>) -> T {
    let mut greatest = input_vec[0];
    
    for item in input_vec {
        if item > greatest {
            greatest = item;
        }
    }
    greatest
}
