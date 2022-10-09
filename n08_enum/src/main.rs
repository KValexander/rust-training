/* Enumerate */
#[derive(Debug)]
enum Ip {
    V4,
    V6
}

/* Struct */
#[derive(Debug)]
struct IpAddr {
    kind: Ip,
    address: String
}

/* Enumerate */
#[derive(Debug)]
enum IpAddrEn {
    V4(String),
    V6(String)
}

/* Enumerate */
#[derive(Debug)]
enum IpAddrEn1 {
    V4(u8, u8, u8, u8),
    V6(String)
}

/* Struct Ipv4Addr */
#[derive(Debug)]
struct Ipv4Addr {
    address: (u8, u8, u8,u8)
}

/* Struct Ipv6Addr */
#[derive(Debug)]
struct Ipv6Addr {
    address: String
}

/* Enumerate */
#[derive(Debug)]
enum IpAddrEn2 {
    V4(Ipv4Addr),
    V6(Ipv6Addr)
}

/* Option enumerate */
#[derive(Debug)]
enum Option<T> {
    None,
    Some(T)
}

/* Enumerate for match */
#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
    // etc
}

/* Enumerate for match */
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State)
}

/* Entry point */
fn main() {

    /* Enumerate */

    /* Ipv4 */
    let home = IpAddr {
        kind: Ip::V4,
        address: String::from("127.0.0.1")
    };

    /* Ipv6 */
    let loopback = IpAddr {
        kind: Ip::V6,
        address: String::from("::1")
    };

    /* Out */
    println!("{:#?}\n{:#?}", home, loopback);

    /* Ipv with enumerate */
    let home_en = IpAddrEn::V4(String::from("127.0.0.1"));
    let loopback_en = IpAddrEn::V6(String::from("::1"));
    println!("{:?}\n{:?}", home_en, loopback_en);

    /* Ipv with enumerate1 */
    let home_en1 = IpAddrEn1::V4(127, 0, 0, 1);
    let loopback_en1 = IpAddrEn1::V6(String::from("::1"));
    println!("{:?}\n{:?}", home_en1, loopback_en1);

    /* Ipv with enumerate2 */
    let home_en2 = IpAddrEn2::V4(Ipv4Addr {
        address: (127, 0, 0, 1)
    });
    let loopback_en2 = IpAddrEn2::V6(Ipv6Addr {
        address: String::from("::1")
    });
    println!("{:?}\n{:?}", home_en2, loopback_en2);

    /* Option enumerate */
    let some_number = Some(5);
    let some_char = Some('e');
    println!("{:?}\n{:?}", some_number, some_char);

    /* Match */
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other),
        // _ => reroll(),
        _ => (),
    }

    fn add_fancy_hat() { println!("add fancy hat") }
    fn remove_fancy_hat() { println!("remove fancy hat") }
    fn move_player(num_spaces: u8) { println!("Move player: {num_spaces}") }
    fn reroll() { println!("Reroll") }

    /* Next match - if */
    let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("Max: {}", max),
    //     _ => ()
    // }
    if let Some(max) = config_max {
        println!("Max: {}", max);
    }

    /* Next match - if else */
    let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State: {:?}", state),
    //     _ => count += 1
    // }
    // if let Coin::Quarter(state) = coin {
    //     println!("State: {:?}", state);
    // } else {
    //     count += 1;
    // }
    
}

/* Value in cents */
fn value_in_cents(coin : Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State: {:?}", state);
            25
        },
    }
}