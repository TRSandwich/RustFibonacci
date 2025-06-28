use std::process::exit;

fn main() {
    let mut x: i32 = 0;
    let mut y: i32 = 1;
    let mut z: i32 = 0;
    let i: i32 = 1; // variable for while loop

    println!("{}", z); // prints initial value, 0

    while i == 1 { // infinite loop
        while z <= 2600 { // actual loop
            println!("{}", x);

            z = x + y;
            x = y;
            y = z;
        }
        
        exit(0); // exits program
    }
}