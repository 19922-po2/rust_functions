use std::fs;
use std::io; // Add itertools = "0.13" in Cargo.toml

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

#[allow(unused)]
pub fn string_count_unscrambled() {
    let mut input: String = String::new(); // create a mutable empty String
    println!("Word:");
    io::stdin() // access standard input
        .read_line(&mut input) // read into the string
        .expect("Failed to read line"); // handle errors

    // read_line leaves a newline `\n` at the end, so trim it
    let input: &str = input.trim();

    let mut chars: Vec<char> = input.chars().collect();
    let mut results = Vec::new();

    // Compute len BEFORE borrowing chars mutably
    let n = chars.len();
    permute(&mut chars, 0, n - 1, &mut results);

    results.sort();
    results.dedup();

    for word in &results {
        println!("{}", word);
    }
    // Print total count
    println!("Total permutations: {}", results.len());
}

fn permute(chars: &mut Vec<char>, l: usize, r: usize, results: &mut Vec<String>) {
    if l == r {
        results.push(chars.clone().into_iter().collect());
    } else {
        for i in l..=r {
            chars.swap(l, i);
            permute(chars, l + 1, r, results);
            chars.swap(l, i);
        }
    }
}

#[allow(unused)]
pub fn recursive_backwards() {
    let mut input: String = String::new(); // create a mutable empty String
    println!("Word:");
    io::stdin() // access standard input
        .read_line(&mut input) // read into the string
        .expect("Failed to read line"); // handle errors

    // read_line leaves a newline `\n` at the end, so trim it
    let input: &str = input.trim();
    let mut backwards = reverse_string(input);

    fn reverse_string(s: &str) -> String {
        // Base case: empty string or single character
        if s.len() <= 1 {
            return s.to_string();
        }

        // Recursive case:
        // - Take the first character
        // - Reverse the rest of the string
        // - Append the first character at the end
        let first = &s[0..1]; // first character
        let rest = &s[1..]; // rest of the string
        format!("{}{}", reverse_string(rest), first)
    }

    println!("{} backwards is {}.", input, backwards)
}

#[allow(unused)]
pub fn rot13() {
    match fs::read_to_string("src/r13-test.txt") {
        Ok(contents) => {
            println!("File contents: {}", contents);

            let mut result = String::new();

            for c in contents.chars() {
                result.push((((c as u8 - b'a' + 13) % 26) + b'a') as char);
            }

            println!("Rot-13 encryption: {}", result);
            fs::write("src/un-r13-test.txt", result);
        }
        Err(e) => println!("Error: {}", e),
    }
}

#[allow(unused)]
pub fn un_rot13() {
    match fs::read_to_string("src/un-r13-test.txt") {
        Ok(contents) => {
            println!("File contents: {}", contents);

            let mut result = String::new();

            for c in contents.chars() {
                result.push((((c as u8 - b'a' + 13) % 26) + b'a') as char);
            }

            println!("Rot-13 encryption: {}", result);
            fs::write("src/r13-test.txt", result);
        }
        Err(e) => println!("Error: {}", e),
    }
}
