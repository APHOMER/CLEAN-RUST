// COMMON COLLECTION TYPES => VECTORS, UTF8, HASH MAPS
// VECTORS - UTF8 - Hashmaps
// COLLECTION TYPES => VECTORS => Vec<T>
// Vectors can only store values of the same type.



// std ===> standard collection

  
fn main() {
    println!("Hello, world!");
//     let _v:Vec<i32> = Vec::new();

//     // Macro to create a vector of numbers
//     let mut _v:Vec<i32> = Vec::new();
//     let mut _v:Vec<i32> = vec![1,2,3];

//     _v.push(5);
//     _v.push(6);
//     _v.push(7);
//     _v.push(8);
//     _v.push(9);

//     println!("{:?}", _v);

//     let fifth: &i32 = &_v[4]; // Direct Indexing.

//     println!("The fifth element is {fifth}");


// // GET METHOD
//     let sixth: Option<&i32> = _v.get(5);
//     match sixth {
//         Some(sixth) => println!("The sixth element is {sixth}"),
//         None => println!("There is no sixth element."),
//     }



    
// ---------------COLLECTION-TYPE--------------------
// ......................UTF8-------------------------
    
    // Mutate the variable [push to it]
    let mut s: String = String::from("");
    s.push_str("APHO");
    s.push_str("MER"); // to push characters more than one.     
    s.push('!'); // Push is used to print single character.     

    println!("This project is owned by {s}");

    // If you want to combine strings, use the + operator
    let greet1: String = String::from("GOOD ");
    let greet2: String = String::from("BYE !");

    let greet_all: String = greet1 + &greet2; // note, s1 has been moved here and can no longer be used.

    println!("Hello sir, I just wanted to say {}", greet_all);


    // ------------------------HASHMAPS-------------------
   
    use std::collections::HashMap;

    let mut payroll = HashMap::new();

    payroll.insert(String::from("Aphomer"), 2500);
    payroll.insert(String::from("Mercy"), 5100);
    payroll.insert(String::from("Kayrose"), 1500);

    let kay_name: String = String::from("Kayrose");
    let kay_cash: i32 = payroll.get(&kay_name).copied().unwrap_or(0);

    
    for (name, amount) in &payroll {
        println!("{name} is collecting ${amount} per month.");

    }
    
    println!("{kay_cash}");
    println!("{kay_name}");

}







