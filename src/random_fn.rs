use chrono::Local;
use rand::Rng;

#[allow(unused)]
pub fn pick_random_fn() {
    // Generate a random number 0, 1, or 2
    let choice = rand::thread_rng().gen_range(0..3);

    match choice {
        0 => greet(),
        1 => show_date(),
        2 => tell_joke(),
        _ => unreachable!(),
    }
}

// Function 1
pub fn greet() {
    println!("Hello! Have a great day!");
}

// Function 2
pub fn show_date() {
    let now = Local::now();
    println!("Today's date and time is: {}", now.format("%Y-%m-%d %H:%M:%S"));
}

// Function 3
pub fn tell_joke() {
    println!("Why do Rust programmers never fear memory leaks? Because they always own their data!");
}
