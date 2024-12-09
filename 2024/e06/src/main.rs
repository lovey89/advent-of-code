use std::fs;

const INPUT_FILE: &str = "input.txt";

#[derive(PartialEq, Clone, Copy)]
enum Position {
    Obstacle,
    Visited(bool, bool, bool, bool), // up, down, left, right
    Unvisited,
}

impl From<char> for Position {
    fn from(value: char) -> Self {
        match value {
            '.' => Position::Unvisited,
            '#' => Position::Obstacle,
            '^' => Position::Visited(true, false, false, false),
            _ => panic!("Should not appear in input"),
        }
    }
}

impl Position {
    fn is_visited_before(self, dir: &Direction) -> bool {
        let (up, down, left, right) = match self {
            Position::Visited(up, down, left, right) => (up, down, left, right),
            _ => return false,
        };
        match dir {
            Direction::Up => up,
            Direction::Down => down,
            Direction::Left => left,
            Direction::Right => right,
        }
    }

    fn visit_position(self, dir: &Direction) -> Position {
        let (up, down, left, right) = match self {
            Position::Visited(up, down, left, right) => (up, down, left, right),
            _ => (false, false, false, false),
        };
        match dir {
            Direction::Up => Position::Visited(true, down, left, right),
            Direction::Down => Position::Visited(up, true, left, right),
            Direction::Left => Position::Visited(up, down, true, right),
            Direction::Right => Position::Visited(up, down, left, true),
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&self) -> Direction {
        match self {
            Direction::Up => Self::Right,
            Direction::Down => Self::Left,
            Direction::Left => Self::Up,
            Direction::Right => Self::Down,
        }
    }

    fn next_coordinate(&self, (y, x): (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up => {
                if y == 0 {
                    None
                } else {
                    Some((y - 1, x))
                }
            }
            Direction::Down => Some((y + 1, x)),
            Direction::Left => {
                if x == 0 {
                    None
                } else {
                    Some((y, x - 1))
                }
            }
            Direction::Right => Some((y, x + 1)),
        }
    }
}

fn read_input(filename: &str) -> (Vec<Vec<Position>>, (usize, usize)) {
    let map: Vec<Vec<Position>> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().map(Into::into).collect())
        .collect();

    let mut start_position = None;
    'outer: for (y, row) in map.iter().enumerate() {
        for (x, pos) in row.iter().enumerate() {
            if matches!(pos, Position::Visited(..)) {
                start_position = Some((y, x));
                break 'outer;
            }
        }
    }
    (map, start_position.unwrap())
}

fn get_visited_map(
    map: &Vec<Vec<Position>>,
    (mut current_y, mut current_x): (usize, usize),
) -> Option<Vec<Vec<Position>>> {
    let mut map = map.to_owned();
    let mut dir = Direction::Up;
    let height = map.len();
    let width = map[0].len();
    loop {
        let next_coord = dir.next_coordinate((current_y, current_x));
        let Some((y_next, x_next)) = next_coord else {
            break;
        };
        if y_next >= height || x_next >= width {
            break;
        }
        let current_position = map[y_next][x_next];
        if current_position == Position::Obstacle {
            dir = dir.turn();
            continue;
        }
        if current_position.is_visited_before(&dir) {
            return None;
        }
        map[y_next][x_next] = current_position.visit_position(&dir);
        current_y = y_next;
        current_x = x_next;
    }
    Some(map)
}

fn part_1() {
    let (map, (current_y, current_x)) = read_input(INPUT_FILE);
    let visited_map = get_visited_map(&map, (current_y, current_x))
        .expect("The unaltered map should not contain any loops");
    let res: i32 = visited_map
        .into_iter()
        .map(|line| {
            TryInto::<i32>::try_into(
                line.into_iter()
                    .filter(|p| matches!(p, Position::Visited(..)))
                    .count(),
            )
            .unwrap()
        })
        .sum();
    println!("{res}");
}

fn part_2() {
    let (mut map, (current_y, current_x)) = read_input(INPUT_FILE);
    let visited_map = get_visited_map(&map, (current_y, current_x))
        .expect("The unaltered map should not contain any loops");

    let mut visited_positions: Vec<(usize, usize)> = vec![];
    for (y, row) in visited_map.iter().enumerate() {
        for (x, coord) in row.iter().enumerate() {
            if matches!(coord, Position::Visited(..)) {
                visited_positions.push((y, x));
            }
        }
    }
    let res = visited_positions
        .into_iter()
        .filter(|(y, x)| {
            //let mut modified_map = map.clone();
            map[*y][*x] = Position::Obstacle;
            let visited_map = get_visited_map(&map, (current_y, current_x));
            map[*y][*x] = Position::Unvisited;
            visited_map.is_none()
        })
        .count();
    println!("{res}");
}

fn main() {
    use std::time::Instant;
    println!("Day 6");
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
