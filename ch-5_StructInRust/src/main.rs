#[derive(Debug)]
struct User {
    username: String,
    email: String,
    signed_in_count: u64,
    active: bool,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(&self) -> u32 {
        self.height * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

// Multiple Implementation can be possible
impl Rectangle {
    fn extraFunc(area: u32) -> Rectangle {
        Rectangle { width: 50, height: 70 }
    }
}
fn main() {
    // basic_struct_check();
    struct_manage();
}
fn struct_manage() {
    let rec = Rectangle {
        width: 20,
        height: 30
    };
    println!("Main Square Info: {:#?}, Square: {}", rec, rec.square());
    let rec1 = Rectangle {
        width: 10,
        height: 20,
    };
    let rec2 = Rectangle {
        width: 30,
        height: 40
    };
    println!("Compare Struct: {}, {}", rec.can_hold(&rec1), rec.can_hold(&rec2));
    let rec3 = Rectangle::extraFunc(90);
    println!("Extra Func Output: {:#?}", rec3);
}
fn basic_struct_check() {
    let user1 = User {
        email: String::from ("hrudaya21@gmail.com"),
        username: String::from("hrudaya1"),
        signed_in_count: 1,
        active: true,
    };
    println!("Value of User1:  {:#?}", user1);
    let user2 = User {
        email: String::from("hrudaya1@gmail.com"),
        username: String::from("hrudaya2"),
        ..user1
    };
    println!("Value of User2:  {:#?}", user2);
    let user3 = build_user(String::from("hrudaya2@gmail.com"), String::from("hrudaya3"));
    println!("Value of User3:  {:#?}", user3);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        signed_in_count: 1,
    }
}
