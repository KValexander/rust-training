/* Struct Rectangle */
struct Rectangle {
    width: u32,
    height: u32,
}

/* Impl Rectangle */
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height 
    }
}

/* Add two */
pub fn add_two(a: i32) -> i32 {
    a + 2
}

/* Greeting */
pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

/* Struct Guess */
struct Guess {
    value: i32,
}

/* Impl Guess */
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess {}", value);
        }
        Guess { value }
    }
}

/* Start - cargo test */
#[cfg(test)]
mod tests {
    use super::*;

    /* It adds two */
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    /* Greeting contains name */
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    /* Check guess */
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    /* It works */
    #[test]
    fn it_works() -> Result<(), String> {
        if 2+2 == 4 { Ok(()) } else { Err(String::from("plus plus")) }
    }

    /* Exploration */
    #[test]
    #[ignore]
    fn exploration() {
        assert_eq!(2 + 2, 4)
    }

    /* Another */
    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }

    /* Larger can hold smaller */
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 5,
        };
        assert!(larger.can_hold(&smaller));
    }

    /* Smaller cannot hold larger */
    #[test]
    fn smaller_cannot_hold_larger() {
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

}
