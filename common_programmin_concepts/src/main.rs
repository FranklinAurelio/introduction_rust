const CREATOR_PROGRAMER: &str = "Franklin";

fn main() {
    let mut x = 5;
    println!("The values of x is {x}");
    x = 6;
    println!("the values of x now is {x}");
    shadowing_example();
    shadowing_example_mut();
    println!("The value const is {CREATOR_PROGRAMER}");
    println!("-------------------------------- Data types ------------------------");
    operations();
    tuple();
    array();
    loop_test();
    while_test();
    for_loop();
}
//let for default are immutables but using the sufix mut, they are mutables, but the prefix const are always immutables
//Shadowing
fn shadowing_example() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn shadowing_example_mut() {
    let mut x = 5;

    x += 1;

    {
        x *= 2;
        println!("The value of x in the inner scope with mut is: {x}");
    }

    println!("The value of x with mut is: {x}");
}

// um numero assinado é um numero que pertece aos inteiros, já se o numero pertence apenas aos naturais ele é não assinado portanto
//na hora de definir a tipagem, usamos i para numeros assinados e u para não assinados como por exemple um numero de 32bits (i32, u32)

fn operations() {
    // addition
    let sum = 5 + 10;
    println!("the addition is {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("the subtraction is {difference}");

    // multiplication
    let product = 4 * 30;
    println!("the multiplication is {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("the quotient is {quotient}");
    let truncated = -5 / 3; // Results in -1
    println!("the truncated is {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("the remainder is {remainder}");
}

//tuplas
fn tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("the values of tuple are {x}, {y}, {z}");
    let value_in_index = tup.1;
    println!("the tuple in index 2 is {value_in_index}")
}

//array
fn array() {
    let array_values: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let first = array_values[0];
    let size_of_array = array_values.len();
    println!("the first item is {first}");
    println!("the lenth is {size_of_array}");
}

// loop
fn loop_test() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn while_test() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
    for number in 0..10 {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
