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
