use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.trim();

    let mut index1 = 0;
    let mut index2 = 0;

    for (i, _) in input.chars().enumerate() {
        let recent: Vec<String> = input[i..i + 4].chars().map(|c| c.to_string()).collect();

        if (1..recent.len()).any(|i| recent[i..].contains(&recent[i - 1])) {
            continue;
        } else {
            let char = recent.join("");
            index1 = input.find(&char).unwrap() + 4;

            break;
        }
    }

    for (i, _) in input.chars().enumerate() {
        let recent: Vec<String> = input[i..i + 4].chars().map(|c| c.to_string()).collect();

        if (1..recent.len()).any(|i| recent[i..].contains(&recent[i - 1])) {
            continue;
        } else {
            let char = recent.join("");
            index2 = input.find(&char).unwrap() + 14;

            break;
        }
    }

    println!("Part 1: {}\nPart 2: {}", index1, index2);
}
