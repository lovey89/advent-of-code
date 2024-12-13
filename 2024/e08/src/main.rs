use std::collections::{
    HashMap,
    HashSet,
};
use std::fs;

use itertools::Itertools;

const INPUT_FILE: &str = "input.txt";

fn to_i32(n: usize) -> i32 {
    n.try_into().unwrap()
}

fn read_input(filename: &str) -> (HashMap<char, Vec<(i32, i32)>>, i32, i32) {
    let mut antenna_positions_by_frequency: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let file_content = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '.' {
                        return;
                    }
                    antenna_positions_by_frequency
                        .entry(c)
                        .or_default()
                        .push((to_i32(y), to_i32(x)));
                })
                .count()
        })
        .collect::<Vec<_>>();
    (
        antenna_positions_by_frequency,
        to_i32(file_content.len()),
        to_i32(file_content[0]),
    )
}

fn get_antenna_pairs(antenna_positions: Vec<(i32, i32)>) -> Vec<((i32, i32), (i32, i32))> {
    antenna_positions
        .into_iter()
        .permutations(2)
        .map(|f| {
            let mut iter = f.into_iter();
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .collect()
}

fn valid_pos((y, x): (i32, i32), height: i32, width: i32) -> bool {
    if y < 0 || x < 0 || y >= height || x >= width {
        return false;
    }
    true
}

fn part_1() {
    let (antenna_positions_by_frequency, height, width) = read_input(INPUT_FILE);
    let mut res: HashSet<(i32, i32)> = HashSet::new();
    for (_, antenna_positions) in antenna_positions_by_frequency {
        let antenna_pairs = get_antenna_pairs(antenna_positions);
        for ((y_0, x_0), (y_1, x_1)) in antenna_pairs {
            let y_diff = y_1 - y_0;
            let x_diff = x_1 - x_0;
            let npos_0 = (y_1 + y_diff, x_1 + x_diff);
            let npos_1 = (y_0 - y_diff, x_0 - x_diff);
            if valid_pos(npos_0, height, width) {
                res.insert(npos_0);
            }
            if valid_pos(npos_1, height, width) {
                res.insert(npos_1);
            }
        }
    }
    println!("{}", res.len());
}

fn part_2() {
    let (antenna_positions_by_frequency, height, width) = read_input(INPUT_FILE);
    let mut res: HashSet<(i32, i32)> = HashSet::new();
    for (_, antenna_positions) in antenna_positions_by_frequency {
        let antenna_pairs = get_antenna_pairs(antenna_positions);
        for ((y_0, x_0), (y_1, x_1)) in antenna_pairs {
            let y_diff = y_1 - y_0;
            let x_diff = x_1 - x_0;
            for i in 1.. {
                let npos = (y_1 + i * y_diff, x_1 + i * x_diff);
                if !valid_pos(npos, height, width) {
                    break;
                }
                res.insert(npos);
            }
            for i in 1.. {
                let npos = (y_1 - i * y_diff, x_1 - i * x_diff);
                if !valid_pos(npos, height, width) {
                    break;
                }
                res.insert(npos);
            }
        }
    }
    println!("{}", res.len());
}

fn main() {
    use std::time::Instant;
    println!("Day 8");
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
