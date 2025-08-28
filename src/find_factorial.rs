use std::io;

pub fn find_factorial() -> usize {
    let mut input = String::new(); // create a mutable empty String
    println!("Number:");
    io::stdin() // access standard input
        .read_line(&mut input) // read into the string
        .expect("Failed to read line"); // handle errors

    // read_line leaves a newline `\n` at the end, so trim it
    let input = input.trim().parse().expect("Please type a number!\n");

    fn calc_factorial(input: usize) -> usize {
        if input == 0 {
            1
        } else {
            input * calc_factorial(input - 1)
        }
    }

    println!("Factorial of {input} is: {}", calc_factorial(input));
    return input;
}
