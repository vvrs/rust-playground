fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);

    // x = 6;

    // println!("The value of x is: {}", x);

    // // Difference between let and const is that const is always immutable and `mut` cannot be used.
    // const MAX_POINTS: u32 = 100_000;

    // let y = 7;

    // // Shadowing
    // let y = 8;

    // // Datatypes

    // // Integers
    // // Floating-Point
    // // Booleans
    // // Characters

    // // Default is i32

    // // different ways to represent integers
    // let decimal = 98_222;
    // let hex = 0xff;
    // let octal = 0o77;
    // let binary = 0b1111_0000;
    // let byte = b'A';

    // // default floating point is f64

    // // Tuples
    // // Compund types
    // let tup: (i32, f64, u8) = (500, 6.4, 1);

    // // Destructuring
    // let (x, y, z) = tup;

    // calling funtions
    another_function();

    println!("The value of y is: {}", statements_and_expressions());
}

// Functions
fn another_function() {
    println!("Another function.");
}

// Statements and Expressions
fn statements_and_expressions() -> i32 {
    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resulting value.
    // Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    // return value of y is 4
    return y;
}

// Control Flow
fn control_flow_example() {
    // if
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // loop
    // loop {
    //     println!("again!");
    // }

    // while
    // let mut number = 3;

    // while number != 0 {
    //     println!("{}!", number);

    //     number = number - 1;
    // }

    // println!("LIFTOFF!!!");

    // for
    // let a = [10, 20, 30, 40, 50];

    // for element in a.iter() {
    //     println!("the value is: {}", element);
    // }

    // for number in (1..4).rev() {
    //     println!("{}!", number);
    // }

    // println!("LIFTOFF!!!");
}
