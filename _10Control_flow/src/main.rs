
#![allow(warnings)]

fn main() {
    let name: String = "Mercy".to_string();
    let age: u16 = 22;

    // if name == "Mercy" {
    //     println!("You can talk to Daniella");
    // } else {
    //     println!("You can only talk to Kayrose.")
    // }

    // if age >= 22 {
    //     println!("You can talk to Daniella");
    // } else {
    //     println!("You can only talk to Kayrose.")
    // }

    
    // Multiple Conditions with else if:
    let number = 18;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    // Using if in a let statement
    let work = false;
    let digit = if work {4} else {7};
    println!("It is {work} that we worked {digit} times yesterday.");

}
