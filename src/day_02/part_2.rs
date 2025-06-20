use std::fs;

pub fn read_file() {
    let input = fs::read_to_string("src/day_02/input").unwrap();
    
    let mut safe_reports = Vec::new();
    let mut unsafe_reports = Vec::new();

    for line in input.lines() {
        let split: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if is_safe(split.clone(), false) {
            safe_reports.push(split)
        } else {
            unsafe_reports.push(split)
        }
    }

    println!("Safe ->: {:?}", safe_reports);
    println!("Unsafe ->: {:?}", unsafe_reports);
    println!("Result = {}", safe_reports.len());
}

fn is_safe(levels: Vec<i32>, is_skip_used: bool) -> bool {
    let mut last_diff: i32 = 0;

    let mut i = 0;

    loop {
        if i+1 >= levels.len() {
            return true
        }

        let diff = levels[i+1] - levels[i];

        if diff.abs() > 3 || diff == 0 || (last_diff != 0 && (last_diff / last_diff.abs()) != (diff / diff.abs())) {
            if is_skip_used {
                return false
            }
            let mut levels_clone_1 = levels.clone();
            let mut levels_clone_2 = levels.clone();
            levels_clone_1.remove(i);
            levels_clone_2.remove(i+1);
            if i == 1 {
                let mut levels_clone_0 = levels.clone();
                levels_clone_0.remove(0);
                return is_safe(levels_clone_0, true) || is_safe(levels_clone_1, true) || is_safe(levels_clone_2, true) 
            }
            return is_safe(levels_clone_1, true) || is_safe(levels_clone_2, true)
        }

        last_diff = diff;
        i += 1;
    }
}