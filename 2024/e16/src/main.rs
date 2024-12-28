use std::cmp::{
    self,
    Ordering,
};
use std::collections::{
    BinaryHeap,
    HashMap,
    HashSet,
};
use std::fs;

const INPUT_FILE: &str = "input.txt";
//const INPUT_FILE: &str = "mini.txt";

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn next_coord(&self, (y, x): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (y - 1, x),
            Direction::South => (y + 1, x),
            Direction::West => (y, x - 1),
            Direction::East => (y, x + 1),
        }
    }
    fn any_sides_is_open(&self, (y, x): (usize, usize), map: &[Vec<Tile>]) -> bool {
        match self {
            Direction::North | Direction::South => {
                map[y][x - 1] != Tile::Wall || map[y][x + 1] != Tile::Wall
            }
            Direction::West | Direction::East => {
                map[y - 1][x] != Tile::Wall || map[y + 1][x] != Tile::Wall
            }
        }
    }
    fn get_opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
    previous_posistion: Option<(usize, usize)>,
    direction: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| self.direction.cmp(&other.direction))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn read_input(filename: &str) -> (Vec<Vec<Tile>>, (usize, usize), Direction) {
    let mut start_pos = (0, 0);
    let map = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start_pos = (y, x);
                    }
                    c.into()
                })
                .collect()
        })
        .collect();
    (map, start_pos, Direction::East)
}

fn part_1() {
    let (map, start_position, start_direction) = read_input(INPUT_FILE);

    let mut heap = BinaryHeap::new();

    heap.push(State {
        cost: 0,
        position: start_position,
        previous_posistion: None,
        direction: start_direction,
    });
    let mut visited_positions = HashSet::new();

    while let Some(State {
        cost,
        position: (y, x),
        direction,
        ..
    }) = heap.pop()
    {
        if !visited_positions.insert((y, x)) {
            // The set already contained this value
            continue;
        }
        if map[y][x] == Tile::End {
            println!("{}", cost);
            return;
        }
        // Find neighbors
        for dir in [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ] {
            if dir == direction.get_opposite() {
                continue;
            }
            let mut last_pos = (y, x);
            let mut additional_cost = if dir == direction { 1 } else { 1001 };
            loop {
                let (y_new, x_new) = dir.next_coord(last_pos);
                let tile = map[y_new][x_new];
                if tile == Tile::Wall {
                    // We hit a wall
                    break;
                }
                if tile == Tile::End || dir.any_sides_is_open((y_new, x_new), &map) {
                    heap.push(State {
                        cost: cost + additional_cost,
                        position: (y_new, x_new),
                        previous_posistion: None,
                        direction: dir,
                    });
                    break;
                }
                last_pos = (y_new, x_new);
                additional_cost += 1;
            }
        }
    }
}

fn find_best_positions(
    (y, x): (usize, usize),
    backtrack: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    best_positions: &mut HashSet<(usize, usize)>,
    handled_paths: &mut HashSet<(usize, usize, usize, usize)>,
) {
    let previous_positions = backtrack.get(&(y, x));
    if previous_positions.is_none() {
        return;
    }
    for (prev_y, prev_x) in previous_positions.unwrap() {
        if !handled_paths.insert((y, x, *prev_y, *prev_x)) {
            continue;
        }
        if y == *prev_y {
            for new_x in cmp::min(*prev_x, x)..=cmp::max(*prev_x, x) {
                best_positions.insert((y, new_x));
            }
        } else {
            for new_y in cmp::min(*prev_y, y)..=cmp::max(*prev_y, y) {
                best_positions.insert((new_y, x));
            }
        }
        find_best_positions((*prev_y, *prev_x), backtrack, best_positions, handled_paths);
    }
}

fn part_2() {
    let (map, start_position, start_direction) = read_input(INPUT_FILE);

    let mut heap = BinaryHeap::new();

    heap.push(State {
        cost: 0,
        position: start_position,
        previous_posistion: None,
        direction: start_direction,
    });
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    let mut backtrack: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    let mut end_position = (0, 0);

    while let Some(State {
        cost,
        position: (y, x),
        previous_posistion,
        direction,
    }) = heap.pop()
    {
        if let Some(d) = dist.get(&(y, x)) {
            if cost > *d {
                // This path is worse than the one we already found
                continue;
            } else if previous_posistion.is_some() {
                backtrack
                    .get_mut(&(y, x))
                    .unwrap()
                    .push(previous_posistion.unwrap());
                continue;
            }
        } else {
            dist.insert((y, x), cost);
        };
        if previous_posistion.is_some() {
            backtrack.insert((y, x), vec![previous_posistion.unwrap()]);
        }
        if map[y][x] == Tile::End {
            println!("{}", cost);
            end_position = (y, x);
            break;
        }
        // Find neighbors
        for dir in [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ] {
            if dir == direction.get_opposite() {
                continue;
            }
            let mut last_pos = (y, x);
            let mut additional_cost = if dir == direction { 1 } else { 1001 };
            loop {
                let (y_new, x_new) = dir.next_coord(last_pos);
                let tile = map[y_new][x_new];
                if tile == Tile::Wall {
                    // We hit a wall
                    break;
                }
                if tile == Tile::End || dir.any_sides_is_open((y_new, x_new), &map) {
                    heap.push(State {
                        cost: cost + additional_cost,
                        position: (y_new, x_new),
                        previous_posistion: Some((y, x)),
                        direction: dir,
                    });
                }
                last_pos = (y_new, x_new);
                additional_cost += 1;
            }
        }
    }

    let mut best_positions = HashSet::new();
    let mut handled_paths = HashSet::new();
    find_best_positions(
        end_position,
        &backtrack,
        &mut best_positions,
        &mut handled_paths,
    );
    println!("{}", best_positions.len())
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
