/* Use */
use std::collections::HashMap;

/* Enum */
#[derive(Debug)]
enum Spread {
    Int(i32),
    Float(f64),
    Text(String)
}

/* Entry point */
fn main() {

    /* Vector */
    let mut v : Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];

    /* Push elements in vector */
    v.push(4);
    v.push(5);
    v.push(6);

    /* Read vector */
    let third : &i32 = &v[2];
    println!("The third element is {}", third);

    let third : Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element")
    }

    /* Get vector */
    // let doesnt_exist = &v[100]; // error, end programm
    let doesnt_exist = v.get(100); // return None
    println!("{:?}", doesnt_exist);

    // let first = &v[0];
    // v.push(6);
    // println!("First: {}", first); // error - v is not in first, v is v.push(6)

    /* Enumerate vector */
    for i in &mut v {
        println!("before: {}", i);
        *i += 10; // accessing a vector element
        println!("after: {}", i);
    }

    /* Vector with enum */
    let row = vec![
        Spread::Int(3),
        Spread::Float(3.3),
        Spread::Text(String::from("String")),
    ];
    println!("{:?}", row);

    /* Create string */
    // let mut s = String::from("String");
    let mut s = String::new();
    s.push_str("Strin"); // push &str
    s.push('g'); // push char
    // s.push("g") // error
    // s.push("String") // error
    // s.push('String') // error
    println!("{}", s);

    /* Concatenation string */
    let s1 = String::from("Hello, "); 
    let s2 = String::from("world!"); 
    let s3 = s1 + &s2; // s1 move to s3, s2 not
    // println!("{}", s1); // error
    println!("{}", s2); // success
    println!("{}", s3); // success

    /* Format string */
    let s1 = String::from("tic"); 
    let s2 = String::from("tac"); 
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s1); // succes
    println!("{}", s2); // succes
    println!("{}", s3); // succes
    println!("{}", s); // succes

    /* Slice string */
    println!("{}-{}-{}", &s[..3], &s[4..7], &s[8..11]);

    /* Enumerate string */
    for c in s.chars() {
        println!("{}", c);
    }

    /* Hash map */
    let mut scores = HashMap::new();
    // hashmap.insert(name, value);
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // scores.insert(String::from("Blue"), 20); // rewrite blue
    scores.entry(String::from("Green")).or_insert(30);
    println!("{:?}", scores);

    /* Read hash map */
    let team = String::from("Blue");
    let score = scores.get(&team).copied().unwrap_or(0);
    println!("{}", score);

    /* Enumerate hash map */
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    /* Create new value based on old value */
    let text = "Something text something text";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);


}
