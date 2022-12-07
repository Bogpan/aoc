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
        .map(|s| {
            s.into_iter()
                .filter(|i| *i != "[!]")
                .map(|c| c.trim_matches(&['[', ']'][..]))
                .collect()
        })
        .collect();

    crate_stacks1.iter_mut().for_each(|c| c.reverse());

    let mut crate_stacks2 = crate_stacks1.clone();

    for line in second.lines() {
        let indices: Vec<usize> = line
            .split(' ')
            .filter_map(|c| c.parse::<usize>().ok())
            .collect();

        move_crates(
            &mut crate_stacks1[..],
            (indices[0], indices[1] - 1, indices[2] - 1),
            true,
        );

        move_crates(
            &mut crate_stacks2[..],
            (indices[0], indices[1] - 1, indices[2] - 1),
            false,
        );
    }

    let top_part1: String = crate_stacks1.iter().map(|s| *s.last().unwrap()).collect();
    let top_part2: String = crate_stacks2.iter().map(|s| *s.last().unwrap()).collect();

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

fn move_crates(vec: &mut [Vec<&str>], (amount, from, to): (usize, usize, usize), rev: bool) {
    let size = vec[from].len();
    let elements = vec[from].drain(size - amount..);

    let removed: Vec<&str> = if rev {
        elements.rev().collect()
    } else {
        elements.collect()
    };

    vec[to].extend(removed);
}
