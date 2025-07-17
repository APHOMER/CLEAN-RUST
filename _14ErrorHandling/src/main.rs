// APPROACH1 -> Result<T,E>
// APPROACH2 -> Option<T>


    // Approach 1
    // enum Option<T> { // Define the generic Option type
    //     Some(T), //Reperesents a value
    //     None, // Represent no value
    // }

fn divideOption(numerator: f64, denominator:f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }   
}

    // Approach 2
    // enum Result<T, E> { // Define the generic Result type
    //     Ok(T), // Represents a value
    //     Err(E), // Represents an error
    // }

fn divideResult(numerator: f64, denominator: f64) -> Result<f64, String> {   
    if denominator == 0.0 {
        Err("Cannot divide by 0".to_string())
    } else {
        Ok(numerator / denominator)
    }
}



fn main() {
    let outcome = divideOption(10.0, 50.0);
    match outcome {
        Some(x) => println!("DivideOprion_Result: {}", x),
        None => println!("Cannot divide by Zero"),
    }


    // println!("Result is {}", result);

    match divideResult(100.0, 0.0) {
        Ok(result) => println!("DivideResult: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}










