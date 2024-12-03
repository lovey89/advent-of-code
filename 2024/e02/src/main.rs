use std::fs;

const INPUT_FILE: &str = "input.txt";

fn read_input(filename: &str) -> Vec<Vec<i32>> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn report_is_safe(report: &[i32]) -> bool {
    let ascending = (report[1] - report[0]).signum();

    for i in 1..report.len() {
        let new_diff = report[i] - report[i - 1];
        let abs_diff = new_diff.abs();
        if !(1..=3).contains(&abs_diff) || new_diff.signum() != ascending {
            return false;
        }
    }
    true
}

fn dampened_report_is_safe(report: &[i32]) -> bool {
    // Naive approach. Just try all different combinations. No need to try the non-altered report. If it is valid, then
    // it should be valid if we also remove the first element
    for i in 0..report.len() {
        let mut cloned_report = report.to_owned();
        cloned_report.remove(i);
        if report_is_safe(&cloned_report) {
            return true;
        }
    }
    false
}

fn part_1() {
    let reports = read_input(INPUT_FILE);
    let filtered_reports: Vec<Vec<i32>> = reports
        .into_iter()
        .filter(|report| report_is_safe(report))
        .collect();
    println!("{}", filtered_reports.len());
}

fn part_2() {
    let reports = read_input(INPUT_FILE);
    let filtered_reports: Vec<Vec<i32>> = reports
        .into_iter()
        .filter(|report| dampened_report_is_safe(report))
        .collect();
    println!("{}", filtered_reports.len());
}

fn main() {
    part_1();
    part_2();
}
