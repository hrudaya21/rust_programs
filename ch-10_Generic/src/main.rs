

// x in Point_differentType struct of type T (may be integer)
// y in Point_differentType struct of the U (may be float)
// Here bot x and y can be same type or different type.
#[derive(Debug)]
struct Point_differentType<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point_differentType<T, U> {
    fn mixup<V, W>(self, other: Point_differentType<V, W>) -> Point_differentType<T, W> {
        Point_differentType {
            x: self.x,
            y: other.y,
        }
    }

}
struct Point<T> {
    x: T,
    y: T,
}
// Implementation of the Struct Point of Type T
impl<U> Point<U> {
    fn x(&self) -> &U {
        &self.x
    }
}
impl<f64> Point<f64> {
    fn y(&self) -> &f64 {
        &self.x
    }
}
fn check_generic_struct_2() {
    let p1 = Point {x: 10.0, y: 20.0};
    // p1 has access to both x and y function
    let p2 = Point {x: 10, y: 20};
    // p2 has access to only x function.
}
fn main() {
    check_generic_basic();
    check_generic_struct_1();
    check_generic_struct_2();
}
fn check_generic_struct_1() {
    // 
    let p1 = Point_differentType { x: 5, y: 10.0 };
    let p2 = Point_differentType {x: 5.0, y: 10.0};
    let p3 = Point_differentType {x: 5.0, y: 10};
    println!("P1: {:?}", p1);
    println!("P2: {:?}", p2);
    println!("P3: {:?}", p3);
}
fn check_generic_basic() {
    let int_vec = vec![80, 10, 100, 90, 20, 50];
    let largest = find_greater_number(int_vec);
    println!("Int Greatest Number: {:?}", largest);
    let char_vec = vec!['a', 'z', 'c', 'd', 'e'];
    let largest = find_greater_number(char_vec);
    println!("Char Greatest Number: {:?}", largest);
}
// Here the T Generic Type for the Function has PartialOrd and Copy Trait implementation.
// All generic type can't have greater/lesser (<, >) operations by default. Hence Trait has been taken for generic
fn find_greater_number<T: PartialOrd + Copy>(input_vec: Vec<T>) -> T {
    let mut greatest = input_vec[0];
    
    for item in input_vec {
        if item > greatest {
            greatest = item;
        }
    }
    greatest
}
