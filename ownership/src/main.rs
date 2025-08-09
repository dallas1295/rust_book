mod slices;
fn main() {
    // let s: &str = "hello"; // block-scoped only valid after this point until the end of the function.
    // ^ This is immutable it cannot be changes ^

    // This String function allocates the variable into the heap
    // because we won't know if it is or isn't going to grow
    let mut s = String::from("hello");
    s.push_str(", world");

    println!("{s}");

    // Integers are known values, therefore they are stored on the stack.
    let x = 5;
    let y = x;
    println!("{y}");

    // s2 is not copying s1, instead it is taking ownership s1's pointer to itself. Rust will then make s1
    // After the move, s1 is invalid and cannot be used. This prevents both s1 and s2
    // from trying to free the same heap memory when they go out of scope.
    // that situation would be called a "double free" and has the possibility of exploitation.
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s2}");

    // This is where s4 now owns it's own version of s3's pointer. This is performance-wise
    // expensive
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("{s4}");

    ownership_and_scope();

    let owned = gives_ownership();
    println!("{}", owned);

    references();
} // Now just like JS/TS, Go, and Python, everythign in the function is out of scope and there for
// it is dropped from memory.

fn ownership_and_scope() {
    let s = String::from("hello"); // s is comes into scope.

    takes_ownership(s); // s moves into the function and now is no longer valid.

    let x = 5; // x comes into scope.

    makes_copy(x); // x would move into the fn BUT u32 is Copy trait. So you can still use it.

    println!("{x}"); // this still works.
}

fn takes_ownership(some_string: String) {
    println!("{some_string}")
} // memory freed

fn makes_copy(some_integer: i32) {
    println!("{some_integer}")
} // memory freed

fn gives_ownership() -> String {
    let some_string = String::from("I'm giving ownership");

    some_string
}

fn references() {
    let s1 = String::from("hello");
    let s1_length = calculate_length(&s1); // referenced we're not taking ownership of s1 so it is still useable.
    println!("Here's s1: {}, and it's length: {}", s1, s1_length);

    let mut s = String::from("What's up");
    // this will change the mutable variable
    mut_ref(&mut s);
    // I can now print that variable
    println!("{}", &mut s);

    // you can borrow a mutable reference once.
    let r1 = &mut s;
    // let r2 = &mut s;
    println!("{}", r1);
    // the reason is it will through the borrow checker error I can only borrow the refernce to s
    // one time.

    // Also cannot borrow a mutable reference and an immutable reference of the same variable.
    let q1 = &s; // Okay
    let q2 = &s; // Okay
    // let q3 = &mut s; // NOT Okay.
    // q3 won't pass the borrow checker because the program is already using the immutable
    // references in the same line of code
    println!("Testing the q's 1: {}, 2: {}, and not 3.", q1, q2);

    //However if I use q3 later I can use it as long as it is declared AFTER the last usage of the
    //immutable borrow.
    let q3 = &mut s;
    println!("Testing q3: {}", q3);
}

fn calculate_length(s: &String) -> usize {
    s.len()

    // the value being passed in is borrowed we cannot change the value so code like:
    // s.push_str(", world")
    // ^ this won't compile because we're only dealing with a borrowed value NOT the real value.
}

fn mut_ref(s: &mut String) {
    let changed_string = s.push_str(" dog?");
    changed_string
}
