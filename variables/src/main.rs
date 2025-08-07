fn main() {
    variables();
    data_types();
    expression();

    let num = 7;
    // to implicitly return just remove the semicolon from the final line of code.
    // for example:
    if_divisible_by(num);
    let value = conditional(num);
    println!("{}", value);

    println!("Let's count to ten");
    count_to_10();
}

fn variables() {
    // // x is immutable, IT CANNOT CHANGE AT ALL.
    // let x = 5;
    // println!("The value of x is {}", x);
    // // This won't compile and throughs an LSP error as x is immutable.
    // x = 6;
    // println!("The new value of x is {}", x);
    //
    // this one does work:

    let mut x = 5;
    println!("x is {}", x);

    x = 6;
    println!("new x is {}", x);

    // const MUST have type anotations and is usually all caps.
    // const NUMBER: u64 = 100_000

    // Shadowing: Each `let x = ...` creates a new variable named `x` that replaces (shadows)
    // the previous one. The previous value is no longer accessible, but is not "moved" or
    // immediately removed from memory; it will be dropped at the end of its scope if needed.
    let x = 5;
    let x = x + 1;
    let x = x + 2;

    println!("{}", x); // prints 8

    //Shadowing howver does not allow the variable to change type so a structure like this:
    // let space = "    ";
    // let space = space.len()
    // will give a compile error due to the fact that they are different types, the former
    // definition is a string where the latter is a type of usize(integer)
}

fn data_types() {
    // this gives a compile error as unsigned integers CANNOT be negative.
    // let x: u32 = -5;
    // println!("{}", x);
    // however if we changed this to be a signed integer it will not have an error.
    let x: i32 = -5;
    println!("this is a negative number (a signed integer): {}", x);

    // here's a table:
    // 8-bit   | i8    | u8
    // 16-bit  | i16	 | u16
    // 32-bit	 | i32   | u32
    // 64-bit	 | i64	 | u64
    // 128-bit | i128  | u128
    // arch	   | isize | usize
    //
    // the size is how many bits (this affects how big the number can actually be)
    // also don't forget about floats for decimal usage.
    //
    // Let's test.

    let x = 2.0; // by default this gives f64
    let y: f32 = 3.0;
    let z = x + y;
    println!(
        "The result of adding the floats {} and {} equal {}",
        x, y, z
    );

    // Tuples, cannot change in length and type definitions can be infered or specified.
    let tup: (i32, f32, _) = (500, 5.4, "type infered");

    // I'm not using b and c so I add an underscore to the front to make it unused, but still safe.
    let (a, _b, c) = tup;
    println!("The value of a in tup is: {}", a);
    println!("The undefined type of c is: {}", c);

    // Arrays in Rust must ALL be the same type to ensure safety. Arrays in rust are fixed length.
    // If it's necessary to have the array be mutable use vectors.
    let arr: [u32; 5] = [1, 2, 3, 5, 6];
    println!("the value of the third place in arr is: {}", arr[2]);

    // to create an array that has a bunch of the same values in it you can do this:
    // let bunch_of_ones = [1; 12];
    // this will output [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
}

fn expression() -> bool {
    // If statements syntactically are similar to Go.
    // let x: u32 = 6;
    // if x <= 6 {
    //     println!("see the if works");
    // } else {
    //     println!("this doesn't work now")
    // }

    //what about bools?
    // The must have the check in the if statement.
    // in JS you can do this:
    //     if x { console.log("it's a 6")
    // in Rust you cannot do that you MUST set the return type to a bool.

    let x: u32 = 6;
    if x <= 6 { true } else { false } // look implicit return
}

fn if_divisible_by(number: u32) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn conditional(number: u32) -> bool {
    if number >= 4 { true } else { false } // in order to do conditionals like this the output MUST
    // be the same type or it will not compile.
}

fn count_to_10() {
    let mut count: u32 = 0;
    // standard loop
    loop {
        count += 1;
        println!("{}", count);
        if count == 10 {
            println!("We've counted to ten");
            break;
        }
    }

    // we can also do while loops... that same thing is achievable.

    println!("Now let's count down!");
    count = 10;
    while count >= 1 {
        println!("{}", count);
        count -= 1;
    }
    println!("LIFT OFF!!!!!");

    // or even a for loop!
    println!("\n\n\n\n\nNow let's iterate of indexes of an array");

    let arr = [5, 4, 3, 2, 1];
    for idx in arr {
        println!("{idx}");
    }

    // NOW WITH A COUNTDOWN!!!
    println!("Here's another for loop with a countdown");

    // for context the .rev() method does a reverse count in the order. the '=' makes it inclusive,
    // without it it's exclusive and it'd need to be 1(..6)
    for nums in (1..=5).rev() {
        println!("{nums}");
    }
}
