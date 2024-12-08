use std::fs;

const INPUT_FILE: &str = "input.txt";
//const INPUT_FILE: &str = "mini.txt";

#[derive(PartialEq)]
enum Position {
    Obstacle,
    Visited,
    Unvisited,
}

impl From<char> for Position {
    fn from(value: char) -> Self {
        match value {
            '.' => Position::Unvisited,
            '#' => Position::Obstacle,
            '^' => Position::Visited,
            _ => panic!("Should not appear in input"),
        }
    }
}

fn read_input(filename: &str) -> (Vec<Vec<Position>>, (i32, i32)) {
    let map: Vec<Vec<Position>> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().map(Into::into).collect())
        .collect();

    let mut start_position = None;
    'outer: for (y, row) in map.iter().enumerate() {
        for (x, pos) in row.iter().enumerate() {
            if *pos == Position::Visited {
                start_position = Some((y.try_into().unwrap(), x.try_into().unwrap()));
                break 'outer;
            }
        }
    }
    (map, start_position.unwrap())
}

fn part_1() {
    let (mut map, (mut current_y, mut current_x)) = read_input(INPUT_FILE);
    let (mut y_dir, mut x_dir) = (-1, 0);
    let height = map.len().try_into().unwrap();
    let width = map[0].len().try_into().unwrap();
    loop {
        let y_next = current_y + y_dir;
        let x_next = current_x + x_dir;
        if y_next < 0 || y_next >= height || x_next < 0 || x_next >= width {
            break;
        }
        if map[TryInto::<usize>::try_into(y_next).unwrap()]
            [TryInto::<usize>::try_into(x_next).unwrap()]
            == Position::Obstacle
        {
            if x_dir != 0 {
                (y_dir, x_dir) = (x_dir, 0);
            } else {
                (y_dir, x_dir) = (0, -y_dir);
            }
            continue;
        }
        map[TryInto::<usize>::try_into(y_next).unwrap()]
            [TryInto::<usize>::try_into(x_next).unwrap()] = Position::Visited;
        current_y = y_next;
        current_x = x_next;
    }
    let res: i32 = map
        .into_iter()
        .map(|line| {
            TryInto::<i32>::try_into(line.into_iter().filter(|p| *p == Position::Visited).count())
                .unwrap()
        })
        .sum();
    println!("{res}");
}

fn part_2() {
    todo!()
}

fn main() {
    part_1();
    part_2();
}
