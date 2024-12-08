use std::collections::HashMap;
use std::fs;

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

fn find_next(rules: &HashMap<i32, Vec<i32>>, pages: &Vec<i32>) -> (usize, i32) {
    'outer: for (pos, page) in pages.iter().enumerate() {
        let current_rules = rules.get(page);
        let Some(current_rules) = current_rules else {
            continue;
        };
        for check_page in pages {
            if check_page == page {
                continue;
            };
            if !current_rules.contains(check_page) {
                continue 'outer;
            }
        }
        return (pos, *page);
    }
    panic!()
}

fn faulty_pages_value(rules: &HashMap<i32, Vec<i32>>, mut pages: Vec<i32>) -> (i32, i32) {
    let no_of_pages = pages.len();
    let mut current_page = -1;
    for _ in 0..=(no_of_pages - 1) / 2 {
        let (pos, next_page) = find_next(rules, &pages);
        current_page = next_page;
        pages.remove(pos);
    }
    (0, current_page)
}

fn pages_value(rules: &HashMap<i32, Vec<i32>>, pages: Vec<i32>) -> (i32, i32) {
    let no_of_pages = pages.len();
    for (pos, e) in pages.iter().enumerate() {
        let current_rules = rules.get(e);
        let Some(current_rules) = current_rules else {
            if pos < no_of_pages - 1 {
                return faulty_pages_value(rules, pages);
            }
            break;
        };
        for i in &pages[pos + 1..no_of_pages] {
            if !current_rules.contains(i) {
                return faulty_pages_value(rules, pages);
            }
        }
    }
    (pages[(no_of_pages - 1) / 2], 0)
}

fn main() {
    let (rules, updates) = read_input(INPUT_FILE);
    let (part_1, part_2): (i32, i32) = updates
        .into_iter()
        .map(|pages| pages_value(&rules, pages))
        .fold(
            (0, 0),
            |(sum_valid, sum_faulty), (valid_value, faulty_value)| {
                (sum_valid + valid_value, sum_faulty + faulty_value)
            },
        );
    println!("{part_1}");
    println!("{part_2}")
}
