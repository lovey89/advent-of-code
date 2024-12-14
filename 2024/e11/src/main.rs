use std::collections::HashMap;
use std::fs;

const INPUT_FILE: &str = "input.txt";

fn read_input(filename: &str) -> Vec<u64> {
    fs::read_to_string(filename)
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>()
}

fn number_of_digits(n: u64) -> u32 {
    if n == 0 {
        return 1;
    }
    n.ilog10() + 1
}

fn part_1() {
    let mut stones = read_input(INPUT_FILE);
    for _ in 1..=25 {
        let mut next_gen = vec![];
        for stone in &stones {
            let number_of_digits = number_of_digits(*stone);
            if *stone == 0 {
                next_gen.push(1);
            } else if number_of_digits % 2 == 1 {
                next_gen.push(stone * 2024);
            } else {
                next_gen.push(stone / (10u64.pow(number_of_digits / 2)));
                next_gen.push(stone % (10u64.pow(number_of_digits / 2)));
            }
        }
        stones = next_gen;
    }
    println!("{}", stones.len());
}

fn count_stones_after_iterations(
    stone: u64,
    iterations: usize,
    memoization_map: &mut HashMap<(u64, usize), u64>,
) -> u64 {
    if let Some(cached_result) = memoization_map.get(&(stone, iterations)) {
        return *cached_result;
    }
    if iterations == 0 {
        return 1;
    }

    let number_of_digits = number_of_digits(stone);
    let mut res = 0;
    if stone == 0 {
        res = count_stones_after_iterations(1, iterations - 1, memoization_map);
    } else if number_of_digits % 2 == 1 {
        res = count_stones_after_iterations(stone * 2024, iterations - 1, memoization_map)
    } else {
        res = count_stones_after_iterations(
            stone / (10u64.pow(number_of_digits / 2)),
            iterations - 1,
            memoization_map,
        ) + count_stones_after_iterations(
            stone % (10u64.pow(number_of_digits / 2)),
            iterations - 1,
            memoization_map,
        )
    }
    memoization_map.insert((stone, iterations), res);
    res
}

fn part_2() {
    let stones = read_input(INPUT_FILE);
    let mut memoization_map: HashMap<(u64, usize), u64> = HashMap::new();
    let no_of_stone: u64 = stones
        .into_iter()
        .map(|stone| count_stones_after_iterations(stone, 75, &mut memoization_map))
        .sum();
    println!("{no_of_stone}");
}

fn main() {
    use std::time::Instant;
    println!("Day 11");
    println!("---------------");
    let now = Instant::now();
    part_1();
    let elapsed = now.elapsed();
    println!("part1 : {elapsed:.2?}");
    let now = Instant::now();
    part_2();
    let elapsed = now.elapsed();
    println!("part2 : {elapsed:.2?}");
    println!("---------------");
}
