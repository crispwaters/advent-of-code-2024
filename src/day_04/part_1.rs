use std::fs;



pub fn read_file() {
    let input = fs::read_to_string("src/day_04/input.sample").unwrap();

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|x| x.chars().collect())
        .collect();
    search_grid(&grid);
}

fn search_grid(grid: &Vec<Vec<char>>) {
    for row in 0..grid.len() {
        for column in 0..grid[row].len() {
            search_xmas((row, column), grid);
        }
    }
}

fn search_xmas((row, column): (usize, usize), grid: &Vec<Vec<char>>) -> i32 {
    if grid[row][column] != 'X' { return 0 }
    is_word_found((row, column), (-1, 0), grid);
    is_word_found((row, column), (1, 0), grid);
    is_word_found((row, column), (0, -1), grid);
    is_word_found((row, column), (0, 1), grid);
    is_word_found((row, column), (1, 1), grid);
    is_word_found((row, column), (1, -1), grid);
    is_word_found((row, column), (-1, 1), grid);
    is_word_found((row, column), (-1, -1), grid);
    println!("");

    
    return -1
}

fn is_word_found((row, column): (usize, usize), (row_direction, column_direction): (i32, i32), grid: &Vec<Vec<char>>) -> bool {
    let i_row = row as i32;
    let i_col = column as i32;
    let x = get_cell((i_row, i_col), &grid);
    let m = get_cell(cv((i_row, i_col), (row_direction, column_direction), 1), grid);
    let a = get_cell(cv((i_row, i_col), (row_direction, column_direction), 2), grid);
    let s = get_cell(cv((i_row, i_col), (row_direction, column_direction), 3), grid);
    
    print!("'{x}{m}{a}{s}'");

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