use std::fs;

const INPUT_FILE: &str = "input.txt";

fn read_input(filename: &str) -> Vec<Option<usize>> {
    fs::read_to_string(filename)
        .unwrap()
        .trim()
        .chars()
        .enumerate()
        .flat_map(|(pos, c)| {
            let n: usize = c.to_string().parse().unwrap();
            if pos % 2 == 1 {
                vec![None; n]
            } else {
                vec![Some(pos / 2); n]
            }
        })
        .collect()
}

fn find_first_none(disk_map: &[Option<usize>], start_index: usize) -> usize {
    disk_map
        .iter()
        .skip(start_index)
        .position(Option::is_none)
        .unwrap()
        + start_index
}

fn find_last_some(disk_map: &[Option<usize>], start_index: usize) -> usize {
    let length = disk_map.len();
    let index = length - start_index - 1;
    start_index
        - disk_map
            .iter()
            .rev()
            .skip(index)
            .position(Option::is_some)
            .unwrap()
}

fn calculate_and_print_result(disk_map: Vec<Option<usize>>) {
    let res: usize = disk_map
        .into_iter()
        .enumerate()
        .filter(|(_, f)| f.is_some())
        .map(|(pos, file_id)| pos * file_id.unwrap())
        .sum();

    println!("{res}");
}

fn part_1() {
    let mut disk_map = read_input(INPUT_FILE);
    let mut s = 0;
    let mut e = disk_map.len() - 1;

    loop {
        let none_index = find_first_none(&disk_map, s);
        let some_index = find_last_some(&disk_map, e);
        if none_index >= some_index {
            break;
        }
        disk_map.swap(none_index, some_index);
        s = none_index + 1;
        e = some_index - 1;
    }

    calculate_and_print_result(disk_map);
}

fn find_first_empty_block_of_size(
    disk_map: &[Option<usize>],
    req_block_size: usize,
    max_index: usize,
) -> Option<usize> {
    let mut start_index = 0;
    let mut consecutive_blocks_found = 0;
    for (pos, file_block) in disk_map.iter().enumerate() {
        if pos >= max_index {
            return None;
        }
        if file_block.is_some() {
            consecutive_blocks_found = 0;
            continue;
        }
        if consecutive_blocks_found == 0 {
            start_index = pos;
        }
        consecutive_blocks_found += 1;
        if consecutive_blocks_found == req_block_size {
            return Some(start_index);
        }
    }
    None
}

fn find_file_from_end(
    disk_map: &[Option<usize>],
    file_index: usize,
    start_index: usize,
) -> (usize, usize) {
    let length = disk_map.len();
    let index = length - start_index - 1;
    let last_index_of_file = start_index
        - disk_map
            .iter()
            .rev()
            .skip(index)
            .position(|f| *f == Some(file_index))
            .unwrap();
    let file_size = disk_map
        .iter()
        .rev()
        .skip(length - last_index_of_file - 1)
        .take_while(|f| **f == Some(file_index))
        .count();
    (last_index_of_file - file_size + 1, file_size)
}

fn part_2() {
    let mut disk_map = read_input(INPUT_FILE);
    let mut e = disk_map.len() - 1;
    let mut current_file = disk_map.last().unwrap().unwrap() + 1;

    loop {
        current_file -= 1;

        if current_file == 0 {
            break;
        }
        let (file_index, file_size) = find_file_from_end(&disk_map, current_file, e);
        e = file_index - 1;
        let empty_block = find_first_empty_block_of_size(&disk_map, file_size, file_index);
        let Some(empty_block) = empty_block else {
            continue;
        };

        for i in 0..file_size {
            disk_map.swap(empty_block + i, file_index + i);
        }
    }

    calculate_and_print_result(disk_map);
}

fn main() {
    use std::time::Instant;
    println!("Day 9");
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
