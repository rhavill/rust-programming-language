use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    ip_addr_struct();
    ip_addr_enum();
    ip_addr_enum_different_types();
    ip_addr_standard_library();
    variety_of_types();
    option_enum();
}

fn ip_addr_struct() {

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("In ip_addr_struct, home is  {:?}.", home);

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
     println!("In ip_addr_struct, loopback is  {:?}.", loopback);
}

fn route(ip_kind: IpAddrKind) { 
    println!("route was called with ip_kind of {:?}.", ip_kind);
}

// We attach data to each variant of the enum directly, so there is no need for 
// an extra struct.
fn ip_addr_enum() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    println!("In ip_addr_enum, home is  {:?}.", home);

    let loopback = IpAddr::V6(String::from("::1"));
    println!("In ip_addr_enum, loopback is  {:?}.", loopback);
}

fn ip_addr_enum_different_types() {
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    println!("In ip_addr_enum_different_types, home is  {:?}.", home);

    let loopback = IpAddr::V6(String::from("::1"));
    println!("In ip_addr_enum_different_types, loopback is  {:?}.", loopback);
}

fn ip_addr_standard_library() {
    let home = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    println!("In ip_addr_standard_library, home is  {:?}.", home);

    let loopback = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    println!("In ip_addr_standard_library, loopback is  {:?}.", loopback);

}

fn variety_of_types() {
    #[derive(Debug)]
    struct QuitMessage; // unit struct
    #[derive(Debug)]
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    #[derive(Debug)]
    struct WriteMessage(String); // tuple struct
    #[derive(Debug)]
    struct ChangeColorMessage(i32, i32, i32); // tuple struct

    let quit_struct = QuitMessage;
    println!("In variety_of_types, quit_struct is {:?}.", quit_struct);
    let move_struct = MoveMessage { x: 0, y: 0 };
    println!("In variety_of_types, move_struct is {:?}.", move_struct);
    let write_struct = WriteMessage(String::from("Hello!"));
    println!("In variety_of_types, write_struct is {:?}.", write_struct);
    let color_struct = ChangeColorMessage(0, 0, 0);
    println!("In variety_of_types, color_struct is {:?}.", color_struct);
    /*
    If we used the different structs, as above, which each have their own type,
    we couldn’t as easily define a function to take any of these kinds of 
    messages as we could with the Message enum defined in Listing 6-2, which is
    a single type.
    */
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let quit_enum = Message::Quit;
    println!("In variety_of_types, quit_enum is {:?}.", quit_enum);
    let move_enum = Message::Move {x: 0, y: 0};
    println!("In variety_of_types, move_enum is {:?}.", move_enum);
    let write_enum = Message::Write(String::from("Hello!"));
    println!("In variety_of_types, write_enum is {:?}.", write_enum);
    let color_enum = Message::ChangeColor(0, 0, 0);
    println!("In variety_of_types, color_enum is {:?}.", color_enum);
    /*
    Just as we’re able to define methods on structs using impl, we’re also 
    able to define methods on enums. 
    */
    impl Message {
        fn call(&self) {
            println!("In variety_of_types, call method of Message enum was called.")
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn option_enum() {
    // This enum is Option<T>, and it is defined by the standard library as follows:
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("In option_enum, some_number is {:?}.", some_number);
    println!("In option_enum, some_string is {:?}.", some_string);
    println!("In option_enum, absent_number is {:?}.", absent_number);

    /*
    Because Option<T> and T (where T can be any type) are different types, the
    compiler won’t let us use an Option<T> value as if it were definitely a 
    valid value. For example, this code won’t compile because it’s trying to 
    add an i8 to an Option<i8> (Generally, this helps catch one of the most 
    common issues with null: assuming that something isn’t null when it 
    actually is.):
    */
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;

}