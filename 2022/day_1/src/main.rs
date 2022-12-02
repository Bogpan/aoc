use std::fs;

fn main() {
    // println!(
    //     "CURRENT PATH: | {} |",
    //     std::env::current_dir().unwrap().display()
    // );

    let input = fs::read_to_string("day_1/input.txt").expect("Invalid input!");
    let input = input.trim();

    let elves: Vec<&str> = input.split("\n\n").collect();

    let mut elf_calories = Vec::new();

    for elf in elves {
        let calories = elf.split('\n').map(|x| x.parse::<i32>().unwrap());
        let calorie_sum: i32 = calories.sum();

        elf_calories.push(calorie_sum);
    }

    elf_calories.sort();
    elf_calories.reverse();

    let total: &i32 = &elf_calories[0..3].iter().sum();
    println!("Part 1: {}\nPart 2: {}", &elf_calories[0], total);
}
