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
    let mut count = 0;
    if is_word_found((row, column), (-1, 0), grid) { count += 1;}
    if is_word_found((row, column), (1, 0), grid) { count += 1;}
    if is_word_found((row, column), (0, -1), grid) { count += 1;}
    if is_word_found((row, column), (0, 1), grid) { count += 1;}
    if is_word_found((row, column), (1, 1), grid) { count += 1;}
    if is_word_found((row, column), (1, -1), grid) { count += 1;}
    if is_word_found((row, column), (-1, 1), grid) { count += 1;}
    if is_word_found((row, column), (-1, -1), grid) { count += 1;}
    return count
}

fn is_word_found((row, column): (usize, usize), (row_direction, column_direction): (i32, i32), grid: &Vec<Vec<char>>) -> bool {
    let i_row = row as i32;
    let i_col = column as i32;
    if get_cell((i_row, i_col), &grid) != 'X' { return false }
    if get_cell(cv((i_row, i_col), (row_direction, column_direction), 1), grid) != 'M' { return false }
    if get_cell(cv((i_row, i_col), (row_direction, column_direction), 2), grid) != 'A' { return false }
    if get_cell(cv((i_row, i_col), (row_direction, column_direction), 3), grid) != 'S' { return false }
    
    return true
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