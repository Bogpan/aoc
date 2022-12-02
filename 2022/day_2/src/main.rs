use std::fs;

fn main() {
    let input = fs::read_to_string("day_2/input.txt").expect("Invalid input!");
    let input = input.trim();

    let lines = input.lines();

    let mut score_part1 = 0;
    let mut score_part2 = 0;

    for line in lines {
        let (opponent, you): (&str, &str) = line.split_at(1);
        let you = you.trim();

        match opponent {
            "A" => match you {
                "X" => score_part1 += 1 + 3,
                "Y" => score_part1 += 2 + 6,
                "Z" => score_part1 += 3 + 0,
                _ => println!("Invalid move by you."),
            },
            "B" => match you {
                "X" => score_part1 += 1 + 0,
                "Y" => score_part1 += 2 + 3,
                "Z" => score_part1 += 3 + 6,
                _ => println!("Invalid move by you."),
            },
            "C" => match you {
                "X" => score_part1 += 1 + 6,
                "Y" => score_part1 += 2 + 0,
                "Z" => score_part1 += 3 + 3,
                &_ => println!("Invalid move by you."),
            },
            _ => println!("Invalid move by opponent."),
        }

        match opponent {
            "A" => match you {
                "X" => score_part2 += 0 + 3,
                "Y" => score_part2 += 3 + 1,
                "Z" => score_part2 += 6 + 2,
                _ => println!("Invalid move by you."),
            },
            "B" => match you {
                "X" => score_part2 += 0 + 1,
                "Y" => score_part2 += 3 + 2,
                "Z" => score_part2 += 6 + 3,
                _ => println!("Invalid move by you."),
            },
            "C" => match you {
                "X" => score_part2 += 0 + 2,
                "Y" => score_part2 += 3 + 3,
                "Z" => score_part2 += 6 + 1,
                &_ => println!("Invalid move by you."),
            },
            _ => println!("Invalid move by opponent."),
        }
    }

    println!("Part 1: {}\nPart 2: {}", score_part1, score_part2);
}
