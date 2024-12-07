use std::{collections::HashMap, fs};

const INPUT_FILE: &str = "input.txt";

fn read_input(filename: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let read_to_string = fs::read_to_string(filename).unwrap();
    let mut split = read_to_string.split("\n\n");

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    split
        .next()
        .unwrap()
        .lines()
        .map(|s| {
            let mut line_split = s.split('|');
            (
                line_split.next().unwrap().parse::<i32>().unwrap(),
                line_split.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .for_each(|(left, right)| {
            let rule_list = rules.entry(left).or_default();
            rule_list.push(right);
        });
    let updates = split
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();
    (rules, updates)
}

fn pages_value(rules: &HashMap<i32, Vec<i32>>, pages: Vec<i32>) -> i32 {
    let no_of_pages = pages.len();
    for (pos, e) in pages.iter().enumerate() {
        let current_rules = rules.get(e);
        let Some(current_rules) = current_rules else {
            if pos < no_of_pages - 1 {
                return 0;
            }
            break;
        };
        for i in &pages[pos + 1..no_of_pages] {
            if !current_rules.contains(i) {
                return 0;
            }
        }
    }
    pages[(no_of_pages - 1) / 2]
}

fn part_1() {
    let (rules, updates) = read_input(INPUT_FILE);
    let res: i32 = updates
        .into_iter()
        .map(|pages| pages_value(&rules, pages))
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
