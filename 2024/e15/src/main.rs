use std::fs;

const INPUT_FILE: &str = "input.txt";
//const INPUT_FILE: &str = "mini.txt";

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Wall,
    Box,
    Empty,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Tile::Wall,
            'O' => Tile::Box,
            '.' | '@' => Tile::Empty,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
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
                map[ty][tx] = Tile::Box;
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
                .filter(|(_, tile)| *tile == Tile::Box)
                .map(|(x, _)| y * 100 + x)
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{res}");
}

fn part_2() {
    todo!()
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
