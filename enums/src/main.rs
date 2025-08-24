// time to learn about Enums.

// Declartion
// formerly IpAddrKind
enum IpAddr {
    // we can define like this too.
    // V4,
    // V6,
    V4(String),
    V6(String),
    // in this case Rust's std has a V4 and V6 ip address defintion (Ipv4Addr, Ipv6Addr)
    // but i'm not using here.
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// We can also impl enums
impl Message {
    fn call(&self) {
        // method body
    }
}

// struct IpAddr {
//     // this will use the enum declarations to allow multiple types as long as
//     // they are part of IpAddrKind enum.
//     kind: IpAddrKind,
//     address: String,
// }

fn main() {
    // we can define like this...
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}

// Option as a defintion
// the idea is that there will becases we're they're will be a null value.
// Rust doesn't have null type because of the ability to output null into a non-null situation that
// causes a bug.
// so Option enum exists with Some and None as its type.

fn option_and_matching() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    // The best way to use the Option enum is by pattern matching using match
    // numbers wrapped in Option CANNOT be mathed. they need to be matched through to be used to
    // avoid null pointer errors.
    let res = match some_number {
        Some(x) => format!("The number is {}", x),
        None => String::from("Empty"),
    };

    println!("{}", res);
}

// Another example of pattern matching
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Catch All pattern matching

fn move_player(roll: u8) {}
// fn reroll() {}

fn dice_game() {
    let roll = 9;

    match roll {
        3 => println!("Good job"),
        7 => println!("Bad Job"),
        // _ => reroll(),
        other => move_player(other),
    }
}

// Sometimes pattern matching is useful but we only want to handle a specific case within the
// Option type. Enter if let.

fn if_let() {
    let something = Some(3u8);
    // we use _ because we don't actually care about what something is we only care that something
    // is in the something variable to achieve the match.
    // if it's None that nothing happens
    if let Some(_) = something {
        println!("Hello World")
    };
}
