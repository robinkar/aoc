use std::fs;

fn parse_line(line: &str) -> ((u32, u32), (u32, u32)) {
    let nums: Vec<u32> = line.trim().split(&[',', '-']).flat_map(str::parse).collect();
    ((nums[0], nums[1]), (nums[2], nums[3]))
}

fn parse_input(input: &str) -> impl Iterator<Item = ((u32, u32), (u32, u32))> + '_ {
    input.trim().lines().map(parse_line)
}

fn part1(input: &str) -> usize {
    parse_input(input)
        .map(|((l1, r1), (l2, r2))| (l1 <= l2 && r1 >= r2) || (l2 <= l1 && r2 >= r1))
        .filter(|e| *e)
        .count()
}

fn part2(input: &str) -> usize {
    parse_input(input)
        .map(|((l1, r1), (l2, r2))| l1 <= r2 && l2 <= r1)
        .filter(|e| *e)
        .count()
}

fn main() {
    let input = fs::read_to_string("inputs/day04.txt").expect("Could not read input file");
    let part1 = part1(&input);
    let part2 = part2(&input);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8"#;
        let score = part1(&input);
        assert_eq!(score, 2);
    }
    #[test]
    fn test_part2() {
        let input = r#"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8"#;
        let score = part2(&input);
        assert_eq!(score, 4);
    }
}
