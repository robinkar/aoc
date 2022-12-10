use std::{fs, str::FromStr};

#[derive(Debug)]
enum Instr {
    Noop,
    Addx(i32),
}

impl FromStr for Instr {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut t = s.split(' ').take(2);
        match (t.next(), t.next()) {
            (Some("addx"), Some(v)) => Ok(Instr::Addx(v.parse().map_err(|_| "Not a number")?)),
            (Some("noop"), _) => Ok(Instr::Noop),
            _ => Err("Invalid instruction"),
        }
    }
}

impl Instr {
    fn value(&self) -> (i32, i32) {
        match self {
            Instr::Noop => (1, 0),
            Instr::Addx(v) => (2, *v),
        }
    }
}

fn parse_input(input: &str) -> Vec<Instr> {
    input
        .lines()
        .map(Instr::from_str)
        .collect::<Result<Vec<Instr>, _>>()
        .expect("Invalid input")
}

fn part1(input: &str) -> i32 {
    let instructions = parse_input(input);

    let mut cycle = 0i32;
    let mut reg = 1i32;
    let mut signal = 0i32;

    for instr in instructions.iter() {
        let (cycles, to_add) = instr.value();
        for _ in 0..cycles {
            cycle += 1;
            if cycle == 20 || (cycle - 20) % 40 == 0 {
                signal += cycle * reg;
            }
        }
        reg += to_add;
    }
    signal
}

fn part2(input: &str) -> String {
    let instructions = parse_input(input);

    let mut cycle = 0i32;
    let mut reg = 1i32;
    let mut pixels: Vec<char> = vec![];

    for instr in instructions.iter() {
        let (cycles, to_add) = instr.value();
        for _ in 0..cycles {
            if (reg - cycle % 40).abs() <= 1 {
                pixels.push('#');
            } else {
                pixels.push('.');
            }
            cycle += 1;
        }
        reg += to_add;
    }
    let result = pixels
        .chunks(40)
        .map(|chars| chars.iter().collect())
        .collect::<Vec<String>>()
        .join("\n");
    result
}

fn main() {
    let input = fs::read_to_string("inputs/day10.txt").expect("Could not read input file");
    let part1 = part1(&input);
    let part2 = part2(&input);
    println!("Part 1: {part1}");
    println!("Part 2:\n{part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../inputs/day10_test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 13140);
    }
    #[test]
    fn test_part2() {
        let expected = "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(part2(TEST_INPUT), expected);
    }
}
