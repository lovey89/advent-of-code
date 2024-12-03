use regex::Regex;
use std::fs;

const INPUT_FILE: &str = "input.txt";

fn read_input_1(filename: &str) -> Vec<(i32, i32)> {
    let input = fs::read_to_string(filename).unwrap();
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    re.find_iter(&input)
        .map(|m| m.as_str())
        .map(|s| &s[4..s.len() - 1])
        .map(|s| {
            let mut number_iter = s.split(',').map(|n| n.parse::<i32>().unwrap());
            (number_iter.next().unwrap(), number_iter.next().unwrap())
        })
        .collect()
}

fn read_input_2(filename: &str) -> Vec<(i32, i32)> {
    let input = fs::read_to_string(filename).unwrap();
    let re = Regex::new(r"(mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\))").unwrap();

    let mut take = true;
    re.find_iter(&input)
        .map(|m| m.as_str())
        .filter(|&s| {
            if s == "do()" {
                take = true;
                return false;
            } else if s == "don't()" {
                take = false;
                return false;
            }
            take
        })
        .map(|s| &s[4..s.len() - 1])
        .map(|s| {
            let mut number_iter = s.split(',').map(|n| n.parse::<i32>().unwrap());
            (number_iter.next().unwrap(), number_iter.next().unwrap())
        })
        .collect()
}

fn part_1() {
    let input = read_input_1(INPUT_FILE);
    let result = input.iter().fold(0, |sum, (x, y)| sum + x * y);
    println!("{result}");
}

fn part_2() {
    let input = read_input_2(INPUT_FILE);
    let result = input.iter().fold(0, |sum, (x, y)| sum + x * y);
    println!("{result}");
}

fn main() {
    part_1();
    part_2();
}
