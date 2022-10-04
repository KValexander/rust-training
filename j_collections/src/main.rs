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
    for i in &v {
        println!("{}", i);
    }

}
