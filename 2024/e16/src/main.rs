use std::fs;

const INPUT_FILE: &str = "input.txt";

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Wall,
    Empty,
    End,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Tile::Wall,
            'E' => Tile::End,
            '.' | 'S' => Tile::Empty,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn read_input(filename: &str) -> (Vec<Vec<Tile>>, (usize, usize), Direction) {
    let mut start_pos = (0, 0);
    let map = fs::read_to_string(filename).unwrap()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start_pos = (y,x);
                    }
                    c.into()
                })
                .collect()
        })
        .collect();
    (map, start_pos, Direction::East)
}

fn part_1() {
    todo!();
}

fn part_2() {
    todo!();
}

fn main() {
    use std::time::Instant;
    println!("Day 16");
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
