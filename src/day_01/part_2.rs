use std::fs;
use std::collections::HashMap;

pub fn read_file() {
    let input = fs::read_to_string("src/day_01/input").unwrap();
    
    let mut left_values = Vec::new();
    let mut right_values: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        let split: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        left_values.push(split[0]);
        *right_values.entry(split[1]).or_insert(0) += 1;
    }

    let mut result = 0;

    for l in left_values.iter() {
        result += l * right_values.get(l).unwrap_or(&0);
    }

    // println!("Left ->: {:?}", left_values);
    // println!("Right ->: {:?}", right_values);
    println!("Result = {}", result);
}