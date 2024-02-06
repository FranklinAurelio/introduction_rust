const CREATOR_PROGRAMER: &str = "Franklin";

fn main() {
    let mut x = 5;
    println!("The values of x is {x}");
    x = 6;
    println!("the values of x now is {x}");
    shadowing_example();
    shadowing_example_mut();
    println!("The value const is {CREATOR_PROGRAMER}")
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

    x = x + 1;

    {
        x = x * 2;
        println!("The value of x in the inner scope with mut is: {x}");
    }

    println!("The value of x with mut is: {x}");
}

// um numero assinado é um numero que pertece aos inteiros, já se o numero pertence apenas aos naturais ele é não assinado portanto
//na hora de definir a tipagem, usamos i para numeros assinados e u para não assinados como por exemple um numero de 32bits (i32, u32)
