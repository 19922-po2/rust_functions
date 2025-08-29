use std::io;

#[allow(unused)]
pub fn string_count_vowels_consonants() {
    let mut input: String = String::new(); // create a mutable empty String
    println!("Word:");
    io::stdin() // access standard input
        .read_line(&mut input) // read into the string
        .expect("Failed to read line"); // handle errors

    // read_line leaves a newline `\n` at the end, so trim it
    let input: &str = input.trim();

    let mut input_vowels: i32 = 0;
    let mut input_consonants: i32 = 0;

    for c in input.chars() {
        if "aeiou".contains(c) {
            input_vowels += 1
        } else {
            input_consonants += 1
        }
    }

    println!(
        "The word {} has {} vowels and {} consonants",
        input, input_vowels, input_consonants
    );
}

pub fn string_count_unscrambled() {
    let mut input: String = String::new(); // create a mutable empty String
    println!("Word:");
    io::stdin() // access standard input
        .read_line(&mut input) // read into the string
        .expect("Failed to read line"); // handle errors

    // read_line leaves a newline `\n` at the end, so trim it
    let input: &str = input.trim();
    let number_chars: usize = input.len();
    //let mut results = {};

    let input_chars: Vec<char> = input.chars().collect(); // Convert input to Vec<char>
    for _i in 0..number_chars {
        for c in 0..number_chars - 1 {
            if c > 0 && c < number_chars {
                input.insert(c + 1, input[c]);
                input.remove(c);
                println!("{:}", input)
            }
        }
    }

    //println!("The word {input} can be scrambled in: {:}", input, results);
}
