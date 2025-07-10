fn main() {
    // Shadowing is not the same as marking a variable as mutable.
    // It is using the same variable

    let m = 2; // result to 2

    let m = m + 9; // result to 6

    {
        let m = m * 8; // result to 12
        println!("The result of the inner scope is: {m}");
    }

    println!("The result of the outer scope is: {m}");

    // RE-ASSIGNING A VARIABLE TO ANOTHER YPE ENTIRELY(from SRING to NUMBER)
    let gap = "      ";
    let gap = gap.len();

    println!("The space in gap in {gap}");
}

// LESSON 9 IS _09COMMENT