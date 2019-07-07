#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    create_modify();
    field_init_shorthand();
    struct_update_syntax();
    tuple_structs();
}

fn create_modify() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("In create_modify email is {}.", user1.email);
    user1.email = String::from("anotheremail@example.com");
    println!("In create_modify email is {}.", user1.email);
}

fn field_init_shorthand() {
    let user = build_user(String::from("me@example.com"), String::from("me"));
    println!("In field_init_shorthand, user is {:?}", user);
}

fn build_user(email: String, username: String) -> User {
    // Without using the shorthand notation, our struct would be created like
    // this:
    // User {
    //     email: email,
    //     username: username,
    //     active: true,
    //     sign_in_count: 1,
    // }

    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn struct_update_syntax() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // Without using the update syntax, our struct would be created like
    // this:
    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     username: String::from("anotherusername567"),
    //     active: user1.active,
    //     sign_in_count: user1.sign_in_count,
    // };
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("In struct_update_syntax, user2 is {:?}", user2);
}

fn tuple_structs() {
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("In tuple_structs, black is {:?}", black);
    println!("In tuple_structs, origin is {:?}", origin);
}