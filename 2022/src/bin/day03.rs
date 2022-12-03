use std::{collections::HashSet, fs, iter::FromIterator};

fn char_priority(c: &char) -> u32 {
    // Will panic on invalid input
    if c.is_lowercase() {
        *c as u32 - 'a' as u32 + 1
    } else {
        *c as u32 - 'A' as u32 + 27
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(str::trim)
        .map(|l| l.split_at(l.len() / 2))
        .map(|(l, r)| {
            let left = HashSet::<char>::from_iter(l.chars());
            let right = HashSet::<char>::from_iter(r.chars());
            left.intersection(&right).map(char_priority).sum::<u32>()
        })
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    let lines = input.trim().lines().map(str::trim).collect::<Vec<&str>>();
    lines
        .chunks(3)
        .map(|group| {
            // Panics if input lines were not evenly divisible by 3
            let a = HashSet::<char>::from_iter(group[0].chars());
            let b = HashSet::<char>::from_iter(group[1].chars());
            let c = HashSet::<char>::from_iter(group[2].chars());
            let ab = HashSet::<char>::from_iter(a.intersection(&b).cloned());
            ab.intersection(&c).map(char_priority).sum::<u32>()
        })
        .sum::<u32>()
}

fn main() {
    let input = fs::read_to_string("inputs/day03.txt").expect("Could not read input file");
    let part1 = part1(&input);
    let part2 = part2(&input);
    println!("Part 1: {part1}\nPart 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw"#;
        let score = part1(&input);
        assert_eq!(score, 157);
    }
    #[test]
    fn test_part2() {
        let input = r#"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw"#;
        let score = part2(&input);
        assert_eq!(score, 70);
    }
}
