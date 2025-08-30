use std::io;
const PI: f64 = std::f64::consts::PI;

fn parse_input(input: &str) -> Result<(usize, String), String> {
    // Try to split once on the first space
    let (num_str, text) = input
        .split_once(' ')
        .ok_or_else(|| "Input must contain a number and text".to_string())?;

    // Try to parse the number
    let number: usize = num_str.parse().map_err(|_| "Failed to parse number".to_string())?;

    Ok((number, text.to_string()))
}

fn calc_diameter(value: usize, param: &str) -> f64 {
    match param {
        "radius" => return 2.0 * value as f64,
        "area" => return 2.0 * (value as f64 / PI).sqrt(),
        _ => {
            println!("Invalid data type to calc! [Must be radius/diameter/area]");
            return 0.0;
        }
    }
}

fn calc_area(value: usize, param: &str) -> f64 {
    match param {
        "radius" => return PI * value.pow(2) as f64,
        "diameter" => return PI * ((value / 2).pow(2)) as f64,
        _ => {
            println!("Invalid data type to calc! [Must be radius/diameter/area]");
            return 0.0;
        }
    }
}

fn calc_radius(value: usize, param: &str) -> f64 {
    match param {
        "diameter" => return value as f64 / 2.0,
        "area" => return (value as f64 / PI).sqrt(),
        _ => {
            println!("Invalid data type to calc! [Must be radius/diameter/area]");
            return 0.0;
        }
    }
}

#[allow(unused)]
pub fn calc_circle_data() {
    let mut input: String = String::new(); // create a mutable empty String
    println!("Enter the value and radius/diameter/area: [ex: '20 diameter']");
    io::stdin() // access standard input
        .read_line(&mut input) // read into the string
        .expect("Failed to read line"); // handle errors

    // read_line leaves a newline `\n` at the end, so trim it
    let input: &str = input.trim();

    match parse_input(input) {
        Ok((number, text)) => {
            println!("✅ Parsed: number = {}, text = {}", number, text);
            let number_str: usize = number;
            let text: &str = &text;
            match text {
                "radius" => println!(
                    "Diameter = {} and Area = {}",
                    calc_diameter(number_str, text),
                    calc_area(number_str, text)
                ),
                "diameter" => println!(
                    "Radius = {} and Area = {}",
                    calc_radius(number_str, text),
                    calc_area(number_str, text)
                ),
                "area" => println!(
                    "Radius = {} and Diameter = {}",
                    calc_radius(number_str, text),
                    calc_diameter(number_str, text)
                ),
                _ => println!("Invalid data type to calc! [Must be radius/diameter/area]"),
            }
        }
        Err(err) => {
            println!("❌ Error for input '{}': {}", input, err);
        }
    }
}
