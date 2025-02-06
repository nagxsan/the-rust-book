const MAX_POINTS: u32 = 100_000; // a constant is 'always' immutable and the type must be annotated as well

fn main() {
    // an immutable variable is defined here
    let x = 5;
    println!("The value of x is: {}", x);

    // x = 4; -> this line will throw error as you try to modify the value of an immutable variable

    // defining a mutable variable
    let mut y = 6;
    println!("The value of y is: {}", y);

    y = 7; // this works as the value of y can be changed
    println!("The value of y is: {}", y);

    // const SUM = x + y; -> this is not allowed as the value for constants must be defined at compile time

    // Shadowing:
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x is: {}", x); // this prints 12, this is inner shadowing
    }
    println!("The value of x is: {}", x); // this prints 6

    // NOTE: you can mutate a variables value but not its type
    let spaces = "   ";
    let spaces = spaces.len(); // this is shadowing original spaces variable to be some different data type
    println!("The value of spaces is: {}", spaces);

    let mut spaces = "   ";
    // spaces = spaces.len() is not allowed since you are mutating type from &str to usize
}