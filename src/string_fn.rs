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
