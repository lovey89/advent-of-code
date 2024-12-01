use std::{collections::HashMap, fs};

const INPUT_FILE: &str = "input.txt";

fn read_input(filename: &str) -> (Vec<i32>, Vec<i32>) {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let mut number_pair = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
            (number_pair.next().unwrap(), number_pair.next().unwrap())
        })
        .unzip()
}

fn part_1() {
    let (mut list_0, mut list_1) = read_input(INPUT_FILE);
    list_0.sort_unstable();
    list_1.sort_unstable();

    let res = list_0
        .iter()
        .zip(list_1.iter())
        .fold(0, |sum, (v_0, v_1)| (v_0 - v_1).abs() + sum);
    println!("{res}");
}

fn part_2() {
    let (list_0, list_1) = read_input(INPUT_FILE);
    let mut occurence_map = HashMap::<i32, i32>::new();
    for v in list_1 {
        *occurence_map.entry(v).or_insert(0) += 1;
    }
    let res = list_0
        .iter()
        .fold(0, |sum, v| v * occurence_map.get(v).unwrap_or(&0) + sum);
    println!("{res}");
}

fn main() {
    part_1();
    part_2();
}
