#![allow(warnings)]
//  Structs 
//  Structs are used to name package related values similar to tuples.

fn main() {
    // tuple
    let rect: (i32, i32) = (200, 500);

    // println!("Hello, world!");

    // STRUCT
    struct Student{
        name: String,
        matric_no: i64,
        faculty: String,
        department: String,
        level: i32,
    }

    struct User{
        username: String,
        email: String,
        password: i64,
        active: bool,
        sign_in_account: u64,
    }

    // INSTANCIATE THE STRUCT

    let mut student1: Student = Student{
        name: String::from("Mercy"),
        matric_no: 1704070120,
        faculty: String::from("Engineering"),
        department: String::from("SYstems Engineering"),
        level: 400,
    };

    let mut user1: User = User{
        username: String::from("Aphomer"),
        email: String::from("aaphomer@gmail.com"),
        password: 1704070120,
        active: true,
        sign_in_account: 5,
    };

    user1.username = String::from("Phasionistar");
    student1.department = String::from("ELectrical Engineering");

    println!("User1 name is {}", user1.username);
    println!("User1 email is {}", user1.email);
    println!("User1 password is {}", user1.password);

    println!("Student1 is a {} level of student from the department of {} in faculty of {}",student1.level ,student1.department, student1.faculty);
    println!("Student1 matric Number is {}",student1.matric_no);
    // println!("Student1 is from the faculty of {}",student1.faculty);

    fn build_user(email: String, username: String) -> User {
        User{
            active: true,
            email,
            username,
            sign_in_account: 1,
        }
    }

    // Create Instances from other instances
    let user2: User = User{
        email: String::from("kayrose1@gmail.com"),
        ..user1  // => This will carry all the features of user1 except user email.
    };

    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black: Color = Color(0,0,0);
    let white: Color = Color(255,255,255);

    // Unit-like struct
    struct AlwaysEqual;
    let subject: AlwaysEqual = AlwaysEqual;



}



