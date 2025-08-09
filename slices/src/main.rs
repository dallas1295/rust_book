fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // this will return 5 

    s.clear(); // this clears the s variable to an empty string literal "".

    // word still has the value 5 here, but s no longer has any content that we
    // could meaningfully use with the value 5. we can't use that 5 to find anything from s.
    println!("{}", word);

    let s = String::from("hello world");

    let word_two = sliced_first_word(&s);
    println!(
        "the first word in our new s with the slicing method is: {}",
        word_two
    ); // this will return the first word by calculating the slice of bytes.

    // let's now try with just the read-only owner-less &str.

    let this = "that and the other";
    let this_first = sliced_first_word(&this);

    println!("this {}", this_first);

    // this also applies to numbers in arrays and vectors as well. let's try.

    let nums: [i32; 6] = [1, 3, 4, 6, 9, 10];
    let nums_sliced = &nums[1..4];

    assert_eq!(nums_sliced, [3, 4, 6]);
}

// this just returns a byte value and is useless after teh s goes out of scope or is cleared
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
// this will take the the same logic but instead of returning the tuple we have the if statement
// return the slice of the string up until the loop hits a " " byte.
fn sliced_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        };
    }

    &s[..]
}
