use std::fs;

pub fn read_file() {
    let input = fs::read_to_string("src/day_04/input").unwrap();

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|x| x.chars().collect())
        .collect();
    search_grid(&grid);
}

fn search_grid(grid: &Vec<Vec<char>>) {
    let mut count = 0;
    for row in 0..grid.len() {
        for column in 0..grid[row].len() {
            count += search_xmas((row, column), grid);
        }
    }
    println!("Total: {}", count);
}

fn search_xmas((row, column): (usize, usize), grid: &Vec<Vec<char>>) -> i32 {
    // if grid[row][column] != 'X' { return 0 }
    if get_cell((row as i32, column as i32), grid) != 'A' { return 0 }

    let tl = get_cell(cv((row as i32, column as i32), (-1, -1), 1), grid);
    let tr = get_cell(cv((row as i32, column as i32), (-1, 1), 1), grid);
    let bl = get_cell(cv((row as i32, column as i32), (1, -1), 1), grid);
    let br = get_cell(cv((row as i32, column as i32), (1, 1), 1), grid);

    let letters = [tl, tr, bl, br];

    

    if letters.iter().filter(|&&c| c == 'S').count() != 2 { return 0 }
    if letters.iter().filter(|&&c| c == 'M').count() != 2 { return 0 }

    if tl == br { return 0 }

    return 1
}

fn get_cell((row, column): (i32, i32), grid: &Vec<Vec<char>>) -> char {
    if row < 0 || column < 0 { return '.' }

    let r = row as usize;
    let c = column as usize;
    if r >= grid.len() { return '.' }
    if c >= grid[r].len() { return '.' }
    return grid[r][c]
}

/// Calculate Vector
fn cv((r, c): (i32, i32), (rd, cd): (i32, i32), magnitude: i32) -> (i32, i32) {
    return (r + (rd * magnitude), c + (cd * magnitude))
}