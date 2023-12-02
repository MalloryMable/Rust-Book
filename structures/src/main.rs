fn main() {
    //struct instance ()
    let mut user1 = User { //NOTE: certain fields cannot be mutable all or nothing system
        active: true,
        username: String::from("MalloryMable"),
        email: String::from("mallorymable@proton.me"),
        sign_in_count: 1,
    };

    println!("user:\n\tactive: {}\n\tuser: {}\n\temail: {}\n\tsign in count: {}",
        user1.active,user1.username ,user1.email, user1.sign_in_count);

    user1 = build_user(String::from( "mmcrumbliss@gmail.com" ), String::from( "mmcrumbliss@gmail.com" ));

    println!("updated email: {}", user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    //making a struct from a previous struct(not a great way of updating sign_in_count, might be
    //advisable for things like email or name changes??
    println!("updated email: {}", user2.email);


    //ex of using tuple struct
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let _subject = AlwaysEqual;
    
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//shorthand initalization
fn build_user(email: String, username: String) -> User {  //wish I could send &str and dereference
    User { //note to change sign in count this might need to mut
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

//Tuple structs useful for work sort of like java objects
//can be used to differentiate between tuples of the same structure

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//unit type struct(take no arguments)
struct AlwaysEqual;
