use std::collections::HashSet;
use std::fs;

const INPUT_FILE: &str = "input.txt";

fn read_input(filename: &str) -> Vec<Vec<u32>> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

trait Insertable: IntoIterator {
    fn new() -> Self;
    fn insert(&mut self, item: (i32, i32));
    fn len(&self) -> usize;
    fn iter(self) -> impl Iterator<Item = (i32, i32)>;
}

impl Insertable for Vec<(i32, i32)> {
    fn new() -> Self {
        vec![]
    }

    fn insert(&mut self, item: (i32, i32)) {
        self.push(item);
    }

    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn iter(self) -> impl Iterator<Item = (i32, i32)> {
        self.into_iter()
    }
}

impl Insertable for HashSet<(i32, i32)> {
    fn new() -> Self {
        HashSet::new()
    }

    fn insert(&mut self, item: (i32, i32)) {
        HashSet::insert(self, item);
    }

    fn len(&self) -> usize {
        HashSet::len(self)
    }

    fn iter(self) -> impl Iterator<Item = (i32, i32)> {
        self.into_iter()
    }
}

fn get_height_at_coordiante(
    topographic_map: &[Vec<u32>],
    (y, x): (i32, i32),
    (h, w): (i32, i32),
) -> Option<u32> {
    if y >= 0 && x >= 0 && y < h && x < w {
        return Some(
            topographic_map[TryInto::<usize>::try_into(y).unwrap()]
                [TryInto::<usize>::try_into(x).unwrap()],
        );
    }
    None
}

fn get_trailhead_value<T: Insertable>(
    topographic_map: &[Vec<u32>],
    y_start: usize,
    x_start: usize,
) -> usize {
    let h: i32 = topographic_map.len().try_into().unwrap();
    let w: i32 = topographic_map[0].len().try_into().unwrap();
    let map_size = (h, w);
    let mut to_check = T::new();
    to_check.insert((y_start.try_into().unwrap(), x_start.try_into().unwrap()));

    for expected_height in 1..=9 {
        let mut next_to_check = T::new();
        for (y, x) in to_check.iter() {
            let c = (y - 1, x);
            if Some(expected_height) == get_height_at_coordiante(topographic_map, c, map_size) {
                next_to_check.insert(c);
            }
            let c = (y + 1, x);
            if Some(expected_height) == get_height_at_coordiante(topographic_map, c, map_size) {
                next_to_check.insert(c);
            }
            let c = (y, x - 1);
            if Some(expected_height) == get_height_at_coordiante(topographic_map, c, map_size) {
                next_to_check.insert(c);
            }
            let c = (y, x + 1);
            if Some(expected_height) == get_height_at_coordiante(topographic_map, c, map_size) {
                next_to_check.insert(c);
            }
        }
        to_check = next_to_check;
    }
    to_check.len()
}

fn part<T: Insertable>() {
    let topographic_map = read_input(INPUT_FILE);
    let res: usize = topographic_map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, height)| {
                    if *height != 0 {
                        return 0;
                    }
                    //get_trailhead_value(&topographic_map, y, x)
                    //get_trailhead_value2::<Vec<(i32,i32)>>(&topographic_map, y, x)
                    get_trailhead_value::<T>(&topographic_map, y, x)
                })
                .collect::<Vec<_>>()
        })
        .sum();
    //println!("{topographic_map:?}");
    //let x = get_trailhead_value(&topographic_map, 0, 2);
    println!("{res}");
}

fn part_1() {
    part::<HashSet<(i32, i32)>>();
}

fn part_2() {
    part::<Vec<(i32, i32)>>();
}

fn main() {
    use std::time::Instant;
    println!("Day 10");
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
