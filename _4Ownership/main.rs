


// BORROWING, OWNERSHIP AND REFFERENCING

fn main() {
    let s1 = String::from("RUST");
    let len = calc_len(&s1);
    // let len = s1.len();
    println!("The of the string '{}' is {}.", s1, len);
}

fn calc_len(s: &String) -> usize {
    s.len()
}


// 2. THERE CAN ONLY BE ONE OWNER AT A TIME.

fn main() {
    let s1 = String::from("MERCY");
    let s2 = s1;

    // println!("{}", s1);
    println!("{}", s2);
}

// 3. WHEN THE OWNER GOES OUT OF SCOPE, THE VALUE WILL BE DROPPED.

fn main () {
    let s1 = String::from("APHO");
    let len = calc_len(&s1);
    println!("Lenghth of '{}' is {}.", s1, len);
} // s1 goes out of scope and its value will be dropped.

fn printLost(s: &String) {
    println!("{}", s1);
}

fn calc_len(s: &String) -> usize {
    s.len()
}



