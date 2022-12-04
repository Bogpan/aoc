use std::fs;

fn main() {
    // TODO: Change to day_4/input.txt
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.trim();

    let mut count_part1 = 0;
    let mut count_part2 = 0;

    for line in input.lines() {
        let range_bounds: Vec<i32> = line
            .split(&[',', '-'][..])
            .map(|c| c.parse::<i32>().unwrap())
            .collect();

        let (s1, e1, s2, e2) = (
            range_bounds[0],
            range_bounds[1],
            range_bounds[2],
            range_bounds[3],
        );

        if s1 <= s2 && e1 >= e2 || s2 <= s1 && e2 >= e1 {
            count_part1 += 1;
        }
        if e1 >= s2 && s1 <= e2 {
            count_part2 += 1;
        }
    }

    println!("Part 1: {}\nPart 2: {}", count_part1, count_part2);
}
