use std::fs;

pub fn read_file() {
    let input = fs::read_to_string("src/day_02/input").unwrap();
    
    let mut safe_reports = Vec::new();
    let mut unsafe_reports = Vec::new();

    for line in input.lines() {
        let split: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if is_safe(&split) {
            safe_reports.push(split)
        } else {
            unsafe_reports.push(split)
        }
    }

    // println!("Safe ->: {:?}", safe_reports);
    // println!("Unsafe ->: {:?}", unsafe_reports);
    println!("Result = {}", safe_reports.len());
}

fn is_safe(levels: &Vec<i32>) -> bool {
    let mut last_diff = 0;
    for i in 0..levels.len() - 1 {
        let diff = levels[i+1] - levels[i];
        if diff.abs() > 3 { return false }
        if (diff) == 0 { return false }
        if i > 0 && ((diff < 0 && last_diff > 0) || (diff > 0 && last_diff < 0)) { return false }
        last_diff = diff;
    }
    true
}