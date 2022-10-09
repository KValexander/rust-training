/* Use */
use std::fs::File;
use std::fs;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::net::IpAddr;

/* Custom type */

/* Struct */
pub struct Guess {
    value : i32
}

/* Impl */
impl Guess {
    pub fn new(value : i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

/* Entry point */
fn main() {

    /* Crush programm with out error */
    // panic!("Crash and burn");

    /* File */
    let file = File::open("hello.txt");

    /* Check file */
    /* try catch? callback? what is this? */
    let check_file = match file {
        Ok(file) => file,
        // Err(error) => panic!("Problem opening the file: {:?}", error),
        /* Excuse me? */
        Err(error) => match error.kind() {
            /* Excuse me? */
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };

    /* Other file */
    // let other_file = File::open("hello_world.txt").unwrap();
    let other_file = File::open("hello_world.txt")
        .expect("hello_world.txt should be included in this project");

    /* To panic! or not to panic! */
    let home : IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");

}

/* Read file */
fn read_file(filename : &str) -> Result<String, io::Error> {
    
    /* Open file */
    // let mut file = File::open(filename); // 1

    /* Username */
    // let mut username = String::new(); // 1, 2

    /* File read to string */
    // match file.read_to_string(&mut username)?; // 1

    /* He's fine */
    // File::open(filename)?.read_to_string(&mut username)?; // 2

    /* Ok */
    // Ok(username) // Ok // 1, 2
    /* Ok */

    /* He laughs in your face */
    fs::read_to_string(filename) // 3

}

/* Last char of first line */
fn last_char_of_first_line(text : &str) -> Option<char> {
    /* He's fine */
    text.lines().next()?.chars().last()
}