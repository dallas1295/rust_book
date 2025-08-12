#[derive(Debug)]
// in order to use impl or implemetation you first have to derive the struct/type.
struct Rectangle {
    width: u32,
    height: u32,
}
// from there we create the imp that will define the methods that can be called when we have
// something of that type.
// NOTE it is okay in Rust to have multiple implementations on a single struct BUT it's much easier
// and idiomatic to keep them together.
impl Rectangle {
    // This is a method that can be called on any Rectangle instance.
    // It takes `&self`, meaning it borrows the instance immutably
    // so it can read its data without taking ownership.
    // NOTE: &self is shorthand for self: &Self
    fn area(&self) -> u32 {
        // `self` is actually a &Rectangle and refers to this specific Rectangle instance
        // Under the hood, this is: (*self).width and (*self.height)
        // Rust auto dereferences self to get the &Rectangle's data information.
        self.width * self.height
        // Multiplies the width and height fields to calculate the area.
    }

    fn width(&self) -> bool {
        // Auto-deref happens here too: `(*self).width > 0`
        self.width > 0
    }

    // can_hold() will take the instance of &self (the current Rectangle) and compare it to another
    // &Rectangle we define.
    // NOTE: also notice that we have other: &Rectange. that allows me to use the methods on a
    // different reference in the same way I'm using it on &self!
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // NOTE fn's on an impl in Rust that don't take self as a parameter are called "associated
    // functions". these are often used as constructors on a type/struct.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 40,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectange has a none zero width of {}", rect1.width)
    }

    // NOTE: notice how i don't need to own the information that &rect2 and &rect3 are "borrowing"
    // the read-only version of those variables to make the comparison.
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Now let's try to use the associated fn, square on our Rectangle struct
    let new_sq = Rectangle::square(24);
    print!(
        "This new square has is a {}x{} rectangle.",
        new_sq.width, new_sq.height
    );
}
