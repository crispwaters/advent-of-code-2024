use std::fs;

pub fn read_file() {
    let input = fs::read_to_string("src/day_01/input").unwrap();
    
    let mut left_values = Vec::new();
    let mut right_values = Vec::new();

    for line in input.lines() {
        let split: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        left_values.push(split[0]);
        right_values.push(split[1]);
    }

    left_values.sort();
    right_values.sort();

    let mut result = 0;

    for (l, r) in left_values.iter().zip(right_values.iter()) {
        result += (l - r).abs();
    }

    // println!("Left ->: {:?}", left_values);
    // println!("Right ->: {:?}", right_values);
    println!("Result = {}", result);
}