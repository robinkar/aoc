use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day01.txt").expect("Could not read input file");
    let mut elves: Vec<i32> = input
        .split("\n\n")
        .map(|elf| elf.lines().flat_map(str::parse::<i32>).sum())
        .collect();
    elves.sort_by(|a, b| b.cmp(a));
    let part1 = elves[0];
    let part2: i32 = elves[..3].iter().sum();
    println!("Part 1: {part1}\nPart 2: {part2}")
}
