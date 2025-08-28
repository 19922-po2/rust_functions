use std::io;

#[allow(unused)]
pub fn convert_currency() {
    let mut input = String::new(); // create a mutable empty String
    let mut currency_input = String::new(); // create a mutable empty String
    println!("Amount:");
    io::stdin() // access standard input
        .read_line(&mut input) // read into the string
        .expect("Failed to read line"); // handle errors

    // read_line leaves a newline `\n` at the end, so trim it
    let input = input.trim().parse().expect("Please type a number!\n");

    println!("[1] From € to $");
    println!("[2] From $ to €");
    io::stdin() // access standard input
        .read_line(&mut currency_input) // read into the string
        .expect("Failed to read line"); // handle errors

    let currency_input: usize = currency_input.trim().parse().expect("Please pic a valid option!\n");

    let get_sign = |currency_input: usize| -> char { if currency_input == 1 { '€' } else { '$' } };

    let calc_currency = |input: f32| -> f32 {
        // 1.17$ : 1.00€
        if currency_input == 1 {
            (input as f32 * 1.17) as f32
        } else {
            (input as f32 / 1.17) as f32
        }
    };

    println!(
        "Conversion: {}{} = {}{}",
        input,
        get_sign(currency_input),
        calc_currency(input),
        get_sign(currency_input - 1)
    );
}
