use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.trim();

    let index1 = unique_index(input, 4);
    let index2 = unique_index(input, 14);

    println!("Part 1: {}\nPart 2: {}", index1, index2);
}

fn unique_index(string: &str, offset: usize) -> usize {
    for (i, _) in string.chars().enumerate() {
        let recent: Vec<String> = string[i..i + offset]
            .chars()
            .map(|c| c.to_string())
            .collect();

        if (1..recent.len()).any(|i| recent[i..].contains(&recent[i - 1])) {
            continue;
        } else {
            let char = recent.join("");
            return string.find(&char).unwrap() + offset;
        }
    }

    0
}
