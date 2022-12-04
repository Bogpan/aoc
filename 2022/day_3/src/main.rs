use std::fs;

const PRIORITY: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    // TODO: Change to day_3/input.txt
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.trim();

    let mut sum = 0;

    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);
        let first: Vec<char> = first.chars().collect();

        let common: Vec<char> = second.chars().filter(|c| first.contains(c)).collect();

        sum += PRIORITY.find(|c: char| common.contains(&c)).unwrap() + 1;
    }

    println!("{}", sum);
}
