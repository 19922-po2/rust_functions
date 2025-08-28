use std::io;

#[allow(unused)]
pub fn desc_triangle() {
    let mut input = String::new(); // create a mutable empty String
    println!("Number:");
    io::stdin() // access standard input
        .read_line(&mut input) // read into the string
        .expect("Failed to read line"); // handle errors

    // read_line leaves a newline `\n` at the end, so trim it
    let input: usize = input.trim().parse().expect("Please type a number!\n");

    for x in (0..input + 1).rev() {
        for x in (0..x) {
            print!("{}", '*');
        }
        println!();
    }
}

#[allow(unused)]
pub fn asc_triangle() {
    let mut input = String::new(); // create a mutable empty String
    println!("Number:");
    io::stdin() // access standard input
        .read_line(&mut input) // read into the string
        .expect("Failed to read line"); // handle errors

    // read_line leaves a newline `\n` at the end, so trim it
    let input: usize = input.trim().parse().expect("Please type a number!\n");

    for x in 0..input + 1 {
        for x in (0..x) {
            print!("{}", '*');
        }
        println!();
    }
}
