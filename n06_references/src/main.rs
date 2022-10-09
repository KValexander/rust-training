/* Entry point */
fn main() {
    /* Create string */
    let mut s1 = String::from("Hello");

    /* Change string */
    // change_string(&s1); // error
    change_string(&mut s1);

    /*
        let r1 = &mut s1;
        let r2 = &mut s1;
        println!("{r1}, {r2}"); // error

        {
            let r1 = &mut s1;
        }
        let r2 = &mut s1;
        println!("{r1}, {r2}"); // success

    
        let r1 = &s1; // no problem
        let r2 = &s1; // no problem
        let r3 = &mut s1; // big problem!
        println("{r1}, {r2}, {r3}"); // error

        let r1 = &s1;
        let r2 = &s1;
        println("{r1}, {r2}"); // success
        let r3 = &mut s1;
        println("{r3}"); // success

    */
        
    /* Passing a variable reference to a function */
    let len = calculate_length(&s1);
    
    /* Out */
    println!("The length of '{s1}' is {len}");

    /*  Slice
            examples:
                let s = String::from("Hello world");
                
                let hello = &s[0..5]; // hello = "hello"
                    let hello = &s[..5];
                
                let world = &s[6..11]; // world = "world"
                    let world = &s[6..];

                let hw = &s[..]; // hw = "Hello world"
                
                println!("{s} {hello} {world} {hw}");

                let a = [1, 2, 3, 4, 5];
                let slice = &a[1..3];
                assert_eq!(slice, &[2, 3]);
    */

    let s = String::from("Hello world");
    let word = first_word(&s);
    println!("The first word: {word}");

}

/* Change string */
// fn change_string(s : &String) { // error
fn change_string(s : &mut String) {
    s.push_str(", world!");
}

/* Calculate length */
fn calculate_length(s : &String) -> usize {
    s.len()
}

/* Dangling - error, doesn't work */
// fn dangle() -> &String {
//     let s = String::from("String");
//     &s
// }

/* No dangle - success */
// fn no_dangle() -> String {
//     let s = String::from("String");
//     s
// }

/* First word */
fn first_word(s : &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}