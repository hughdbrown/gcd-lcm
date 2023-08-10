use std::io;
use std::io::Write;

use gcd_lcm::{gcd, lcm};

// Prompt the user for an i64.
fn get_i64(prompt: &str) -> i64 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    return trimmed.parse::<i64>()
        .expect("Error parsing integer");
}

fn main() {
    let a = get_i64("Type first number: ");
    let b = get_i64("Type second number: ");
    println!("gcd({}, {}) = {}", a, b, gcd(a, b));
    println!("lcm({}, {}) = {}", a, b, lcm(a, b));
}
