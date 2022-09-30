/* Entry point */
fn main() {

    /*  String
            methods:
                string.push_str("") - appends string to the end
                string.clone() - string cloning
                string.len() - string length

            examples:
                let s = "string"; // error, methods won't work

                let mut s = String::from("string"); // success
                s.push_str(", string2");
                println!("string: {s}");

                let s1 = String::from("string");

                let s2 = s1;
                println!("s1 = {s1}, s2 = {s2}"); // error, s1 moved to s2

                let s2 = s1.clone();
                println!("s1 = {s1}, s2 = {s2}"); // success
                // s2 is self sufficient

                let s2 = &s1;
                println!("s1 = {s1}, s2 = {s2}"); // success
                // s2 reference s1
                
                let x = 5;
                let y = x;
                println!("x = {x}, y = {y}"); // success, because the base data type

    */

    /* Takes ownership */
    let s = String::from("String"); // complex data type
    takes_ownership(s); // now "s" only works inside a function

    /* Make clone */
    let i = 5; // base data type
    make_clone(i); // "i" continues to operate

    /* Gives ownership */
    let s1 = gives_ownership();
    let s2 = String::from(", ahn...");
    /* Takes and gives back */
    let s3 = takes_and_gives_back(s2);
    println!("{s1}{s3}");

    /* Calculate length */
    let s4 = String::from("String");
    let (s5, len) = calculate_length(s4);
    println!("The length of '{s5}' is {len}");

}

/* Takes ownership */
fn takes_ownership(s : String) {
    println!("{s}");
}

/* Make clone */
fn make_clone(i : i32) {
    println!("{i}");
}

/* Gives ownership */
fn gives_ownership() -> String {
    let s = String::from("Gives ownership");
    s
}

/* Takes and gives back */
fn takes_and_gives_back(s : String) -> String {
    s
}

/* Calculate length */
fn calculate_length(s : String) -> (String, usize) {
    let length = s.len();
    (s, length)
}