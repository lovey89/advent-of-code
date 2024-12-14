use std::collections::{
    HashMap,
    HashSet,
};
use std::fs;

const INPUT_FILE: &str = "input.txt";
//const INPUT_FILE: &str = "mini.txt";

fn read_input(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|row| row.chars().collect())
        .collect::<Vec<_>>()
}

fn belongs_to_relevant_group(
    map: &[Vec<char>],
    group_belonings: &[Vec<u32>],
    y: usize,
    x: usize,
    plant: char,
) -> Option<u32> {
    if map[y][x] == plant {
        return Some(group_belonings[y][x]);
    }
    None
}

fn merge_groups(
    group_belonings: &mut [Vec<u32>],
    group_references: &mut HashMap<u32, HashSet<(usize, usize)>>,
    target_group: u32,
    second_group: u32,
) {
    for (y, x) in group_references.get(&second_group).unwrap() {
        group_belonings[*y][*x] = target_group;
    }
    let second_group_members = group_references.remove(&second_group).unwrap();
    let target_group_member = group_references.get_mut(&target_group).unwrap();
    for member in second_group_members {
        target_group_member.insert(member);
    }
}

fn fence_value(group_members: &HashSet<(usize, usize)>) -> usize {
    group_members
        .iter()
        .map(|(y, x)| {
            let mut fences = 0;
            if !group_members.contains(&(y + 1, *x)) {
                fences += 1;
            }
            if !group_members.contains(&(*y, x + 1)) {
                fences += 1;
            }
            if *y == 0 || !group_members.contains(&(y - 1, *x)) {
                fences += 1;
            }
            if *x == 0 || !group_members.contains(&(*y, x - 1)) {
                fences += 1;
            }
            fences
        })
        .sum()
}

fn fence_sides(group_members: &HashSet<(usize, usize)>) -> usize {
    todo!();
}

fn get_groups_and_members() -> HashMap<u32, HashSet<(usize, usize)>> {
    let map = read_input(INPUT_FILE);

    let mut group_belonings: Vec<Vec<u32>> = vec![];
    let mut group_references: HashMap<u32, HashSet<(usize, usize)>> = HashMap::new();
    let mut unique_group_id = 0;

    for (y, row) in map.iter().enumerate() {
        group_belonings.push(vec![]);
        for (x, plant) in row.iter().enumerate() {
            let left_beloning = if x == 0 {
                None
            } else {
                belongs_to_relevant_group(&map, &group_belonings, y, x - 1, *plant)
            };
            let upper_beloning = if y == 0 {
                None
            } else {
                belongs_to_relevant_group(&map, &group_belonings, y - 1, x, *plant)
            };

            let group_id = match (left_beloning, upper_beloning) {
                (None, None) => {
                    let group_id = unique_group_id;
                    unique_group_id += 1;
                    group_id
                }
                (None, Some(group_id)) => group_id,
                (Some(group_id), None) => group_id,
                (Some(left_group), Some(upper_group)) => {
                    if left_group != upper_group {
                        merge_groups(
                            &mut group_belonings,
                            &mut group_references,
                            upper_group,
                            left_group,
                        );
                    }
                    upper_group
                }
            };
            group_belonings[y].push(group_id);
            group_references.entry(group_id).or_default().insert((y, x));
        }
    }
    group_references
}

fn part_1() {
    let group_references = get_groups_and_members();

    let total_cost: usize = group_references
        .values()
        .map(|group_members| {
            let field_size = group_members.len();
            let fences = fence_value(group_members);
            field_size * fences
        })
        .sum();
    println!("{}", total_cost);
}

fn part_2() {
    let group_references = get_groups_and_members();

    let total_cost: usize = group_references
        .values()
        .map(|group_members| {
            let field_size = group_members.len();
            let fences = fence_sides(group_members);
            field_size * fences
        })
        .sum();
    println!("{}", total_cost);
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
