use std::fs;

//const INPUT_FILE: &str = "input.txt";
const INPUT_FILE: &str = "mini.txt";

#[derive(Debug)]
struct Machine {
    a_x: i32,
    a_y: i32,
    b_x: i32,
    b_y: i32,
    p_x: i32,
    p_y: i32,
}

fn get_machine_moves(s: &str) -> (i32, i32) {
    let mut split = s.split_whitespace();
    split.next();
    split.next();
    let x = split.next().unwrap();
    let x = x[2..x.len() - 1].parse().unwrap();
    (x, split.next().unwrap()[2..].parse().unwrap())
}

fn get_prize_location(s: &str) -> (i32, i32) {
    let mut split = s.split_whitespace();
    split.next();
    let x = split.next().unwrap();
    let x = x[2..x.len() - 1].parse().unwrap();
    (x, split.next().unwrap()[2..].parse().unwrap())
}

fn read_input(filename: &str) -> Vec<Machine> {
    fs::read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(|m| {
            let mut lines = m.lines();
            let (a_x, a_y) = get_machine_moves(lines.next().unwrap());
            let (b_x, b_y) = get_machine_moves(lines.next().unwrap());
            let (p_x, p_y) = get_prize_location(lines.next().unwrap());
            Machine{a_x, a_y, b_x, b_y, p_x, p_y}
        })
        .collect::<Vec<_>>()
}

fn valid_solution(Machine{a_x, a_y, b_x, b_y, p_x, p_y}: &Machine, a_pushes: i32, b_pushes: i32) -> bool {
    if a_x * a_pushes + b_x * b_pushes != *p_x {
        return false;
    }
    a_y * a_pushes + b_y * b_pushes == *p_y
}

fn find_all_valid_solutions(machine: Machine) -> Vec<(i32, i32)> {
    let Machine{a_x, a_y, b_x, b_y, p_x, p_y} = machine;
    let max_a = std::cmp::min(p_x / a_x, 100);

    let mut solutions = vec![];
    for a_pushes in (0..=max_a) {
        let leftover_x = p_x - a_pushes - a_x;
        let b_pushes = std::cmp::min(leftover_x / b_x, 100);
        if b_pushes > 100 {
            continue;
        }
        if valid_solution(&machine, a_pushes, b_pushes) {
            solutions.push((a_pushes, b_pushes));
        }
    };
    solutions
}

fn part_1() {
    let machines = read_input(INPUT_FILE);
    println!("{machines:?}");
}

fn part_2() {
    todo!();
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
