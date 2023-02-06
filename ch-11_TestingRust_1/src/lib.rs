#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
}
pub fn greeting(name: &str) -> String {
    // format!("Hello {}!", name)
    format!("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Hrudaya");
        // assert!(result.contains("Hrudaya"));
        assert!(
            result.contains("Hrudaya"),
            "Greeting doesn't contain name!!!, Value is: {}",
            result
        );
    }
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 3 == 5 {
            Ok(())
        } else {
            Err(String::from("Incorrect Operation!!!"))
        }
    }
}