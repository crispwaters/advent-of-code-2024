use std::fs;
use regex::Regex;

pub fn read_file() {
    let regex = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do(n't)?\(\))").unwrap();

    let input = fs::read_to_string("src/day_03/input").unwrap();

    let mut products = Vec::new();
    let mut is_enabled = true;

    for mat in regex.find_iter(&input) {
        if let Some((a, b)) = extract_numbers(&mat.as_str()) {
            // println!("Found: {}, a={}, b={}", mat.as_str(), a, b);
            if is_enabled {
                products.push(a * b);
            }
        }
        if let Some(enabled) = extract_enable_toggle(&mat.as_str()) {
            is_enabled = enabled;
        }
    }

    println!("{:?}", products);
    let sum: i32 = products.iter().sum();
    println!("Sum: {:?}", sum);

}

fn extract_numbers(input: &str) -> Option<(i32, i32)> {
    let regex = Regex::new(r"(?<a>\d{1,3}),(?<b>\d{1,3})").unwrap();

    if let Some(caps) = regex.captures(input) {
        let a = caps["a"].parse::<i32>().ok()?;
        let b = caps["b"].parse::<i32>().ok()?;
        Some((a, b))
    } else {
        None
    }
}

fn extract_enable_toggle(input: &str) -> Option<bool> {
    // println!("{}", input);
    if input == "do()" { return Some(true); }
    if input == "don't()" { return Some(false); }
    None
}