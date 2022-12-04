use std::fs;

const PRIORITY: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let input = fs::read_to_string("day_3/input.txt").unwrap();
    let input = input.trim();

    let groups: Vec<&str> = input.split('\n').collect();
    let groups: Vec<&[&str]> = groups.chunks(3).collect();

    let mut sum_part1 = 0;
    let mut sum_part2 = 0;

    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);
        let first: Vec<char> = first.chars().collect();

        let common: Vec<char> = second.chars().filter(|c| first.contains(c)).collect();

        sum_part1 += PRIORITY.find(|c: char| common.contains(&c)).unwrap() + 1;
    }

    for group in groups {
        let elf1 = group[0];
        let elf2 = group[1];
        let elf3 = group[2];

        let common: Vec<char> = elf3
            .chars()
            .filter(|c| elf2.contains(*c) && elf1.contains(*c))
            .collect();

        sum_part2 += PRIORITY.find(|c: char| common.contains(&c)).unwrap() + 1;
    }

    println!("Part 1: {}\nPart 2: {}", sum_part1, sum_part2);
}
