use std::fs;

const INPUT_FILE: &str = "input.txt";

#[derive(Debug)]
struct Machine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    p_x: i64,
    p_y: i64,
}

fn get_machine_moves(s: &str) -> (i64, i64) {
    let mut split = s.split_whitespace();
    split.next();
    split.next();
    let x = split.next().unwrap();
    let x = x[2..x.len() - 1].parse().unwrap();
    (x, split.next().unwrap()[2..].parse().unwrap())
}

fn get_prize_location(s: &str) -> (i64, i64) {
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
            Machine {
                a_x,
                a_y,
                b_x,
                b_y,
                p_x,
                p_y,
            }
        })
        .collect::<Vec<_>>()
}

fn valid_solution(
    Machine {
        a_x,
        a_y,
        b_x,
        b_y,
        p_x,
        p_y,
    }: &Machine,
    a_pushes: i64,
    b_pushes: i64,
) -> bool {
    if a_x * a_pushes + b_x * b_pushes != *p_x {
        return false;
    }
    a_y * a_pushes + b_y * b_pushes == *p_y
}

fn find_best_solution_cost(machine: &Machine) -> Option<i64> {
    // There should only be at most one solution when solving two unknown in two linear equations
    let Machine {
        a_x,
        a_y,
        b_x,
        b_y,
        p_x,
        p_y,
    } = machine;

    // Solve the unknowns
    let a_pushes = (p_x * b_y - p_y * b_x) / (a_x * b_y - a_y * b_x);
    let b_pushes = (a_x * p_y - a_y * p_x) / (a_x * b_y - a_y * b_x);

    // Verify the solution since we only work with integers
    if valid_solution(machine, a_pushes, b_pushes) {
        return Some(a_pushes * 3 + b_pushes);
    }
    None
}

fn part_1() {
    let machines = read_input(INPUT_FILE);

    let res: i64 = machines.iter().filter_map(find_best_solution_cost).sum();
    println!("{res}");
}

fn part_2() {
    let machines = read_input(INPUT_FILE);

    let res: i64 = machines
        .into_iter()
        .map(|mut m| {
            m.p_x += 10000000000000;
            m.p_y += 10000000000000;
            m
        })
        .filter_map(|m| find_best_solution_cost(&m))
        .sum();
    println!("{res}");
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
