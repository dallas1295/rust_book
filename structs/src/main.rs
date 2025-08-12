// Structs are just like Go and Typescript. Easy custom type definitions for the program
struct User {
    username: String,
    email: String,
}

// If the struct doesn't need name-values tuples can be used to define information.
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

fn main() {
    let mut me = User {
        username: String::from("blubbly1295"),
        email: String::from("blub@blubby.com"),
    };

    // can edit me by just using dot notation for the part of the struct I want to change (notice
    // that me is also mutable above.
    me.email = String::from("anotherblub@blubby.com");

    // This new User me2 takes part of me and "moves" it's ownership to me too.
    // me is now invalid because it doesn't own the parts we moved to me2.
    // ** This is for memory safety (Go back to ownership if get confused).
    let me2 = User {
        username: String::from("blubblub"),
        ..me
    };

    // however oddly me.username is still valid!
    println!("first me username: {}", me.username);
    println!("second me username: {}", me2.username);
    // I can't call the other elements though:
    // remember type string is on the Heap thusly not having Copy trait.
    // println!("{}", me.email);

    let new_username = String::from("createBlub");
    let new_email = String::from("blub@blub.com");

    let new_user = create_user(new_username, new_email);
    println!(
        "Created username: {}, Created email: {}",
        new_user.username, new_user.email
    );

    // let black = Color(0, 0, 0);
    // let origin = Point(0, 0, 0);

    // let blub = InCode;

    // here's an example of where structs would be useful.
    area_find();
}

fn create_user(username: String, email: String) -> User {
    User { username, email }
}

// refactor the whole program to use the Rectangle struct that has width and height definitions.
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_find() {
    // changed rect1 into a Rectangle.
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of a rectangle is {} square pixels.", area(rect1))
}

// the previous definition works if you split into dot notation BUT this is more idiomatic.
fn area(rect: Rectangle) -> u32 {
    rect.width * rect.height
}
