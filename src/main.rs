use std::process::exit;

fn main() {
    let mut x: i32 = 0;
    let mut y: i32 = 1;
    let mut z: i32 = 0;
    let i: i32 = 1;

    println!("{}", z);

    while i == 1 {
        while z <= 2600 {
            println!("{}", x);

            z = x + y;
            x = y;
            y = z;
        }
        
        exit(0);
    }
}