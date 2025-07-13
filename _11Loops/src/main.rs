fn main() {
    // println!("Hello, world!");
    // Loop keyword

    // WHILE LOOP
    // loop {
    //     println!("THIS IS WHILE LOOP");
    // }



    // let mut counter = 0;
    
    // let result = loop {
    //     counter += 3;

    //     if counter == 33 {
    //         break counter * 3;
    //     }
    // };
    // println!("the counter result is {result}.");


    // // LOOP INSIDE LOOP.
    // let mut figures = 0;

    // 'adding_up: loop {
    //     println!("figures {figures}");
    //     let mut remnant = 10;

    //     loop {
    //         println!("It Remains {remnant}");
    //         if remnant == 8 {
    //             break;
    //         }
    //         if figures == 2 {
    //             break 'adding_up;
    //         }
    //         remnant -= 1;
    //     }
    //     figures += 1;
    // }


    // WHILE LOOP.
    let mut count_down = 9;
    while count_down != 6 {
        println!("counting {count_down}");
        count_down -= 1;
        // break;
    }
    println!("GO ! !! !!!");

    let mut count_up = 1;
    while count_up < 6 {
        println!("Counting {count_up}");
        count_up += 1;
    }
    println!("STOP ! !! !!!");

    // LOOPING THROUGH A COLLECTION WITH LOOP

    let a = [0, 1, 2, 3, 4, 5, 6, 8, 9];
    let b = ["a", "b", "c", "d", "e", "f", "g", "h"];

    for num in a {
        println!("{num}")
    }

    for letter in b {
        println!("{letter}")
    }
}




