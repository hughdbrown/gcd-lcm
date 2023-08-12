use std::io;
use std::io::Write;

use gcd_lcm::{gcd, lcm};

// Prompt the user for an u64.
fn get_u64(prompt: &str) -> u64 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    return trimmed.parse::<u64>()
        .expect("Error parsing integer");
}

fn main() {
    let a = get_u64("Type first number: ");
    let b = get_u64("Type second number: ");
    println!("gcd({}, {}) = {}", a, b, gcd(a, b));
    println!("lcm({}, {}) = {}", a, b, lcm(a, b));
}
