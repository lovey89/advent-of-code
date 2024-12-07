use std::{collections::HashMap, fs};

const INPUT_FILE: &str = "input.txt";

fn read_input(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_xmas(matrix: &Vec<Vec<char>>, y_start: i32, x_start: i32, y_diff: i32, x_diff: i32) -> i32 {
    let mut y = y_start;
    let mut x = x_start;
    let y_max: i32 = matrix.len().try_into().unwrap();
    let x_max: i32 = matrix[0].len().try_into().unwrap();

    let mut found: i32 = 0;

    let next_char = HashMap::from([('X', 'M'), ('M', 'A'), ('A', 'S'), ('S', 'X')]);

    let mut looking_for_char = 'X';

    while y >= 0 && x >= 0 && y < y_max && x < x_max {
        let current_char =
            matrix[TryInto::<usize>::try_into(y).unwrap()][TryInto::<usize>::try_into(x).unwrap()];
        if current_char == looking_for_char || current_char == 'X' {
            if current_char == 'S' {
                found += 1;
            }
            looking_for_char = *next_char.get(&current_char).unwrap();
        } else {
            looking_for_char = 'X';
        }
        x += x_diff;
        y += y_diff;
    }

    found
}

fn part_1() {
    let matrix = read_input(INPUT_FILE);

    let y_max: i32 = matrix.len().try_into().unwrap();
    let x_max: i32 = matrix[0].len().try_into().unwrap();

    let mut res = 0;

    for y in 0..y_max {
        res += find_xmas(&matrix, y, 0, 0, 1);
        res += find_xmas(&matrix, y, x_max - 1, 0, -1);
        res += find_xmas(&matrix, y, 0, 1, 1);
        res += find_xmas(&matrix, y, x_max - 1, 1, -1);
        res += find_xmas(&matrix, y, 0, -1, 1);
        res += find_xmas(&matrix, y, x_max - 1, -1, -1);
    }
    res += find_xmas(&matrix, 0, 0, 1, 0);
    res += find_xmas(&matrix, y_max - 1, 0, -1, 0);
    for x in 1..x_max - 1 {
        res += find_xmas(&matrix, 0, x, 1, 0);
        res += find_xmas(&matrix, y_max - 1, x, -1, 0);
        res += find_xmas(&matrix, 0, x, 1, 1);
        res += find_xmas(&matrix, y_max - 1, x, -1, 1);
        res += find_xmas(&matrix, 0, x, 1, -1);
        res += find_xmas(&matrix, y_max - 1, x, -1, -1);
    }
    res += find_xmas(&matrix, 0, x_max - 1, 1, 0);
    res += find_xmas(&matrix, y_max - 1, x_max - 1, -1, 0);

    println!("{res}");
}

fn is_x_mas(matrix: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    if matrix[y][x] != 'A' {
        return false;
    }
    let ul = matrix[y - 1][x - 1];
    let ur = matrix[y - 1][x + 1];
    let ll = matrix[y + 1][x - 1];
    let lr = matrix[y + 1][x + 1];

    if !((ul == 'M' && lr == 'S') || (ul == 'S' && lr == 'M')) {
        return false;
    }
    (ur == 'M' && ll == 'S') || (ur == 'S' && ll == 'M')
}

fn part_2() {
    let matrix = read_input(INPUT_FILE);

    let y_max: usize = matrix.len();
    let x_max: usize = matrix[0].len();

    let mut res = 0;
    for y in 1..y_max - 1 {
        for x in 1..x_max - 1 {
            if is_x_mas(&matrix, y, x) {
                res += 1;
            }
        }
    }

    println!("{res}");
}

fn main() {
    part_1();
    part_2();
}
