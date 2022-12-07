use std::fs;

fn main() {
    let input = fs::read_to_string("day_5/input.txt").unwrap();
    let input = input.trim();

    let parts: Vec<&str> = input.split("\n\n").collect();
    let (first, second) = (parts[0], parts[1]);

    let first = first.replace("    ", " [!]");

    let mut first: Vec<Vec<&str>> = first
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect();

    let crate_stacks1 = first.split_last_mut().unwrap().1.to_owned();
    let transposed = transpose(&crate_stacks1);

    let mut crate_stacks1: Vec<Vec<&str>> = transposed
        .into_iter()
        .map(|c| c.into_iter().filter(|i| *i != "[!]").collect())
        .collect();

    crate_stacks1.iter_mut().for_each(|c| c.reverse());

    let mut crate_stacks2 = crate_stacks1.clone();

    for line in second.lines() {
        let indices: Vec<usize> = line
            .split(' ')
            .filter_map(|c| c.parse::<usize>().ok())
            .collect();

        let (amount, from, to) = (indices[0], indices[1] - 1, indices[2] - 1);
        let size1 = crate_stacks1.get(from).unwrap().len();
        let size2 = crate_stacks2.get(from).unwrap().len();

        let removed1: Vec<&str> = crate_stacks1[from].drain(size1 - amount..).rev().collect();
        let removed2: Vec<&str> = crate_stacks2[from].drain(size2 - amount..).collect();

        crate_stacks1[to].extend(removed1);
        crate_stacks2[to].extend(removed2);
    }

    let top_part1: String = crate_stacks1
        .iter()
        .map(|s| s.last().unwrap().trim_matches(&['[', ']'][..]))
        .collect();

    let top_part2: String = crate_stacks2
        .iter()
        .map(|s| s.last().unwrap().trim_matches(&['[', ']'][..]))
        .collect();

    println!("Part 1: {}\nPart 2: {}", top_part1, top_part2);
}

fn transpose<'a>(matrix: &'a [Vec<&'a str>]) -> Vec<Vec<&'a str>> {
    let mut transposed: Vec<Vec<&str>> = Vec::new();

    for row in matrix {
        for (i, element) in row.iter().enumerate() {
            if transposed.len() <= i {
                transposed.push(vec![element]);
            } else {
                transposed[i].push(element);
            }
        }
    }

    transposed
}
