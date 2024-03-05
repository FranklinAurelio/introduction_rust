fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    let mut s1 = String::from("hello");
    let s2 = s1.clone();
    s1.push_str(", world");

    println!("s1 = {}, s2 = {}", s1, s2);

    let a = String::from("hello"); // s comes into scope

    takes_ownership(a); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x);

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
    println!("{s3}");

    reference_pointer();

    let mut s_slice = String::from("hello world");
    let mut s_slice_word = String::from("Franklin Aurelio");

    let word = first_word(&s_slice);
    {
        // word will get the value 5
        let word_slice = first_word_slice(&s_slice_word);
        println!("the word with slice is {word_slice}");
    }
    let second_word_slice = second_word_slice(&s_slice_word);

    s_slice.clear();

    println!("{word}");
    println!("the word with slice is {second_word_slice}");
    s_slice_word.clear(); // this empties the String, making it equal to ""
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn reference_pointer() {
    let s1 = String::from("Hello");
    let len = calculate_lenth(&s1);
    println!("The lenth of '{}' is {} ", s1, len)
}
fn calculate_lenth(s: &String) -> usize {
    s.len()
}

//Slice

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }

    &s[..]
}
