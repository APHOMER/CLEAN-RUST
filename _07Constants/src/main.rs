// CONSTANTS

fn main() {
    println!("Hello, world!");
    let mut x = 7;
    
    const Y: i32 = 6;
    // You can never change a CONSTANT to mutable.
    // you must declare a constant with CAPITAL Letter.
    // You are obliged to add the type anotation.
    println!("The value of x is: {}", x);
    println!("The value of Y is {}", Y);
    println!("The value of PI is: {}", PI);
    println!("The value of 3 HOURS in Seconds is {}", THREE_HOURS_IN_SECONDS);

}

// you can declare a constant with a type annotation
const PI: f64 = 3.141592653;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

