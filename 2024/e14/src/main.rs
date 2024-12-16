use std::collections::HashSet;
use std::fs;

const INPUT_FILE: &str = "input.txt";
const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

#[derive(Debug)]
struct Robot {
    p_x: i32,
    p_y: i32,
    v_x: i32,
    v_y: i32,
}

fn read_input(filename: &str) -> Vec<Robot> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let p_string = &split.next().unwrap()[2..];
            let v_string = &split.next().unwrap()[2..];
            let mut p_split = p_string.split(',');
            let mut v_split = v_string.split(',');
            let p_x = p_split.next().unwrap().parse().unwrap();
            let p_y = p_split.next().unwrap().parse().unwrap();
            let v_x = v_split.next().unwrap().parse().unwrap();
            let v_y = v_split.next().unwrap().parse().unwrap();
            Robot { p_x, p_y, v_x, v_y }
        })
        .collect()
}

fn determine_quadrant(x: i32, y: i32) -> Option<usize> {
    if x == WIDTH / 2 || y == HEIGHT / 2 {
        return None;
    }
    if x < WIDTH / 2 {
        if y < HEIGHT / 2 {
            return Some(0);
        }
        return Some(1);
    };
    if y < HEIGHT / 2 {
        return Some(2);
    }
    Some(3)
}

fn print_state(t: i32, robot_positions: HashSet<(i32, i32)>) {
    println!("Time {t}");
    println!("========");
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if robot_positions.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn part_1() {
    let robots = read_input(INPUT_FILE);
    let mut robots_in_quadrant = vec![0; 4];
    for Robot { p_x, p_y, v_x, v_y } in robots {
        // Negative modulo is negative so add width/height and run modulo again
        let n_x = (((p_x + 100 * v_x) % WIDTH) + WIDTH) % WIDTH;
        let n_y = (((p_y + 100 * v_y) % HEIGHT) + HEIGHT) % HEIGHT;
        if let Some(quadrant) = determine_quadrant(n_x, n_y) {
            robots_in_quadrant[quadrant] += 1;
        }
    }
    let res: i32 = robots_in_quadrant.into_iter().product();
    println!("{res}");
}

fn part_2() {
    let robots = read_input(INPUT_FILE);
    'time_loop: for t in 0.. {
        let mut robot_positions: HashSet<(i32, i32)> = HashSet::new();
        for Robot { p_x, p_y, v_x, v_y } in &robots {
            let n_x = (((p_x + t * v_x) % WIDTH) + WIDTH) % WIDTH;
            let n_y = (((p_y + t * v_y) % HEIGHT) + HEIGHT) % HEIGHT;
            if !robot_positions.insert((n_x, n_y)) {
                // A duplicate was found. Jump to next time unit
                // I assume that all robots will be in unique positions when
                // the picture is formed
                continue 'time_loop;
            }
        }
        print_state(t, robot_positions);
        break;
    }
}

fn main() {
    use std::time::Instant;
    println!("Day 14");
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
