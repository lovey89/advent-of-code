use std::fs;

use itertools::Itertools;

const INPUT_FILE: &str = "input.txt";

fn read_input(filename: &str) -> Vec<(u64, Vec<u64>)> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let mut split = line.split(": ");
            let sum: u64 = split.next().unwrap().parse().unwrap();
            let values = split
                .next()
                .unwrap()
                .split(" ")
                .map(|n| n.parse().unwrap())
                .collect();
            (sum, values)
        })
        .collect()
}

fn get_operator_combinations(no_of_operators: usize, operator_choises: u32) -> Vec<Vec<u32>> {
    std::iter::repeat(0..operator_choises)
        .take(no_of_operators)
        .multi_cartesian_product()
        .collect()
}

fn valid_calibration_result(sum: &u64, values: &[u64], operator_choises: u32) -> Option<u64> {
    let no_of_values = values.len();
    let no_of_operators = no_of_values - 1;
    let operator_combinations: Vec<Vec<u32>> =
        get_operator_combinations(no_of_operators, operator_choises);
    for operator_combination in operator_combinations {
        let calculated_sum = &values[1..].iter().zip(operator_combination.iter()).fold(
            values[0],
            |calc_sum, (value, op)| {
                if op == &0u32 {
                    calc_sum + value
                } else if op == &1u32 {
                    calc_sum * value
                } else {
                    calc_sum * 10u64.pow(value.ilog10() + 1) + value
                }
            },
        );
        //println!("Sum {calculated_sum}");
        if calculated_sum == sum {
            //println!("Match");
            return Some(*sum);
        }
    }
    None
}

fn part_1() {
    let input = read_input(INPUT_FILE);
    let res: u64 = input
        .iter()
        .filter_map(|(sum, values)| valid_calibration_result(sum, values, 2))
        .sum();

    println!("{res}");
}

fn part_2() {
    let input = read_input(INPUT_FILE);
    let res: u64 = input
        .iter()
        .filter_map(|(sum, values)| valid_calibration_result(sum, values, 3))
        .sum();

    println!("{res}");
}

fn main() {
    use std::time::Instant;
    println!("Day 7");
    println!("---------------");
    let now = Instant::now();
    part_1();
    let elapsed = now.elapsed();
    println!("part1 : {:.2?}", elapsed);
    let now = Instant::now();
    part_2();
    let elapsed = now.elapsed();
    println!("part2 : {:.2?}", elapsed);
    println!("---------------");
}
