use std::fs;

//const INPUT_FILE: &str = "mini.txt";
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

fn get_operator_combinations(no_of_operators: u32) -> Vec<Vec<char>> {
    (0..2u32.pow(no_of_operators))
        .map(|i| {
            (0..no_of_operators)
                .map(|j| i >> j & 1)
                .map(|b| if b == 0 { '+' } else { '*' })
                .collect()
        })
        .collect()
}

fn get_operator_combinations_2(no_of_operators: u32, operators: &[char], mut vectors: &Vec<char>) -> Vec<Vec<char>> {
    // https://stackoverflow.com/questions/75834334/cartesian-product-of-n-ranges
    (0..2u32.pow(no_of_operators))
        .map(|i| {
            (0..no_of_operators)
                .map(|j| i >> j & 1)
                .map(|b| if b == 0 { '+' } else { '*' })
                .collect()
        })
        .collect()
}

fn valid_calibration_result(sum: &u64, values: &[u64]) -> Option<u64> {
    let no_of_values: u32 = values.len().try_into().unwrap();
    let no_of_operators = no_of_values - 1;
    let operator_combinations: Vec<Vec<char>> = get_operator_combinations(no_of_operators);
    for operator_combination in operator_combinations {
        let calculated_sum = &values[1..].iter().zip(operator_combination.iter()).fold(
            values[0],
            |calc_sum, (value, op)| {
                if op == &'+' {
                    calc_sum + value
                } else {
                    calc_sum * value
                }
            },
        );
        if calculated_sum == sum {
            return Some(*sum);
        }
    }
    None
}

fn part_1() {
    let input = read_input(INPUT_FILE);
    let res: u64 = input
        .iter()
        .filter_map(|(sum, values)| valid_calibration_result(sum, values))
        .sum();

    println!("{res}");
}

fn part_2() {
    todo!();
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
