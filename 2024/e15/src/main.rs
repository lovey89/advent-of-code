use std::fs;

use itertools::Itertools;

const INPUT_FILE: &str = "input.txt";

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Wall,
    LeftBox,
    RightBox,
    Empty,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Tile::Wall,
            'O' => Tile::LeftBox,
            '.' | '@' => Tile::Empty,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for MoveDirection {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(MoveDirection::Up),
            'v' => Ok(MoveDirection::Down),
            '<' => Ok(MoveDirection::Left),
            '>' => Ok(MoveDirection::Right),
            _ => Err("Unexpected char".to_string()),
        }
    }
}

impl MoveDirection {
    fn is_vertical(&self) -> bool {
        *self == MoveDirection::Up || *self == MoveDirection::Down
    }

    fn update_map(&self, map: &mut [Vec<Tile>], (y, x): (usize, usize)) -> (usize, usize) {
        let (candidate_y, candidate_x) = self.new_coord((y, x));

        let tile = map[candidate_y][candidate_x];
        if tile == Tile::Empty {
            return (candidate_y, candidate_x);
        } else if tile == Tile::Wall {
            return (y, x);
        };

        let (mut ty, mut tx) = self.new_coord((candidate_y, candidate_x));
        loop {
            let tile = map[ty][tx];
            if tile == Tile::Wall {
                return (y, x);
            } else if tile == Tile::Empty {
                map[ty][tx] = Tile::LeftBox;
                map[candidate_y][candidate_x] = Tile::Empty;
                return (candidate_y, candidate_x);
            }
            (ty, tx) = self.new_coord((ty, tx));
        }
    }

    fn new_coord(&self, (y, x): (usize, usize)) -> (usize, usize) {
        match self {
            MoveDirection::Up => (y - 1, x),
            MoveDirection::Down => (y + 1, x),
            MoveDirection::Left => (y, x - 1),
            MoveDirection::Right => (y, x + 1),
        }
    }

    fn prepare_update(
        &self,
        map: &[Vec<Tile>],
        (y, x): (usize, usize),
    ) -> Option<Vec<(usize, usize)>> {
        let tile = map[y][x];
        if tile == Tile::Empty {
            return Some(vec![]);
        } else if tile == Tile::Wall {
            return None;
        }
        if self.is_vertical() {
            if tile == Tile::LeftBox {
                let mut res = vec![(y, x), (y, x + 1)];
                res.append(&mut self.prepare_update(map, self.new_coord((y, x)))?);
                res.append(&mut self.prepare_update(map, self.new_coord((y, x + 1)))?);
                Some(res)
            } else {
                let mut res = vec![(y, x - 1), (y, x)];
                res.append(&mut self.prepare_update(map, self.new_coord((y, x - 1)))?);
                res.append(&mut self.prepare_update(map, self.new_coord((y, x)))?);
                Some(res)
            }
        } else {
            let mut res = vec![(y, x)];
            res.append(&mut self.prepare_update(map, self.new_coord((y, x)))?);
            Some(res)
        }
    }

    fn update_map2(&self, map: &mut [Vec<Tile>], (y, x): (usize, usize)) -> (usize, usize) {
        let (candidate_y, candidate_x) = self.new_coord((y, x));
        let Some(updates) = self.prepare_update(map, (candidate_y, candidate_x)) else {
            return (y, x);
        };

        // This could be faster. Instead of using unique from itertools we could keep track of which
        // has already been handled using a hashset
        for (y_0, x_0) in updates.into_iter().unique().rev() {
            let (y_1, x_1) = self.new_coord((y_0, x_0));
            map[y_1][x_1] = map[y_0][x_0];
            map[y_0][x_0] = Tile::Empty;
        }
        (candidate_y, candidate_x)
    }
}

fn read_input(filename: &str) -> (Vec<Vec<Tile>>, Vec<MoveDirection>, (usize, usize)) {
    let input_content = fs::read_to_string(filename).unwrap();

    let mut start_position = (0, 0);
    let mut input_split = input_content.split("\n\n");
    let map = input_split
        .next()
        .unwrap()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, p)| {
                    if p == '@' {
                        start_position = (y, x);
                    }
                    p.into()
                })
                .collect()
        })
        .collect();

    let movements = input_split
        .next()
        .unwrap()
        .chars()
        .filter_map(|c| c.try_into().ok())
        .collect();
    (map, movements, start_position)
}

fn print_map(map: &[Vec<Tile>], (r_y, r_x): (usize, usize)) {
    for (y, row) in map.iter().enumerate() {
        for (x, t) in row.iter().enumerate() {
            if y == r_y && x == r_x {
                print!("@");
                continue;
            }
            match t {
                Tile::Wall => print!("#"),
                Tile::LeftBox => print!("["),
                Tile::RightBox => print!("]"),
                Tile::Empty => print!("."),
            }
        }
        println!()
    }
}

fn part_1() {
    let (mut map, movements, (mut y, mut x)) = read_input(INPUT_FILE);
    for movement in movements {
        (y, x) = movement.update_map(&mut map, (y, x));
    }
    let res = map
        .into_iter()
        .enumerate()
        .map(|(y, line)| {
            line.into_iter()
                .enumerate()
                .filter(|(_, tile)| *tile == Tile::LeftBox)
                .map(|(x, _)| y * 100 + x)
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{res}");
}

fn part_2() {
    let (map, movements, (mut y, mut x)) = read_input(INPUT_FILE);
    let mut map: Vec<Vec<Tile>> = map
        .into_iter()
        .map(|row| {
            row.into_iter()
                .flat_map(|t| match t {
                    Tile::Wall => vec![Tile::Wall, Tile::Wall],
                    Tile::LeftBox => vec![Tile::LeftBox, Tile::RightBox],
                    Tile::Empty => vec![Tile::Empty, Tile::Empty],
                    Tile::RightBox => panic!("Should not occur"),
                })
                .collect()
        })
        .collect();
    x *= 2;
    for movement in movements {
        (y, x) = movement.update_map2(&mut map, (y, x));
    }
    let res = map
        .into_iter()
        .enumerate()
        .map(|(y, line)| {
            line.into_iter()
                .enumerate()
                .filter(|(_, tile)| *tile == Tile::LeftBox)
                .map(|(x, _)| y * 100 + x)
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{res}");
}

fn main() {
    use std::time::Instant;
    println!("Day 15");
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
