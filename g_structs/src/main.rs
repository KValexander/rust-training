/*  Struct
        // struct and impl before fn main()

        // Struct Name
        #[derive(Debug)] // for dbg!() and {?:}
        struct Name {
            prop: data type,
            prop: data type,
            prop: data type,
        };
    
        // Impl Name
        // Add methods for structure
        // May multiple impl for one struct
        impl Name { // name impl == name struct
            fn method(&self) {
                println!("{}", self.prop);
            }
        }
        
        // in fn main()
        example:
            let mut s = Name {
                prop: value,
                prop: value,
                prop: value,
            }
            s.prop = value;

            let s1 = Name {
                prop: value,
                ..s
            }

            s1.method();
*/

/* Struct User */
#[derive(Debug)] // for dbg!() and {?:}
struct User {
    email: String,
    username: String,
    active: bool,
    id: u64
}

/* Struct Rectangle */
#[derive(Debug)] // for dbg!() and {?:}
struct Rectangle {
    width: u32,
    height: u32
}

/*  Impl Rectangle */
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

/* Entry point */
fn main() {

    /* User1 */
    let mut user1 = User {
        email: String::from("example@example.com"),
        username: String::from("User"),
        active: true,
        id: 1,
    };
    user1.email = String::from("emaeema");
    // out_user(&user1); // error, user1 move to user3

    /* User2 */
    let user2 = build_user(String::from("email"), String::from("user2"));
    dbg!(&user2);
    // out_user(user2); // user2 move to out_user
    out_user(&user2); // user2 reference to out_user

    /* User3 */
    let user3 = User {
        email: String::from("My mail"),
        ..user1 // user1 move to user3
    };
    // out_user(user3); // user3 move to out_user
    out_user(&user3); // user3 reference to out_user

    /* Rectangle */
    let scale = 1;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 30
    };
    dbg!(&rect1);

    /* Out area */
    println!("Rectangle area: {}", rect1.area());

}

/* Build user */
fn build_user(email : String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        id: 2
    }
}

/* Out user */
fn out_user(user : &User) {
    println!("{:?}", user);
    // println!("{:#?}", user);
    // println!("{}, {}, {}, {}", user.email, user.username, user.active, user.id);
}