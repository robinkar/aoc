use std::{convert::TryInto, fs, str::FromStr};

struct Instr {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instr {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<usize> = s
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .flat_map(str::parse)
            .collect();
        let [amount, from, to]: [usize; 3] =
            nums.try_into().map_err(|_| "Missing numbers in input")?;
        Ok(Instr {
            amount,
            from: from - 1,
            to: to - 1,
        })
    }
}
fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Instr>) {
    let (initial_state, instructions) = input.split_once("\n\n").expect("Invalid input");
    let n_lines = (initial_state.lines().next().expect("Invalid input").len() + 1) / 4;

    let mut state: Vec<Vec<char>> = vec![vec![]; n_lines];
    initial_state.lines().rev().skip(1).for_each(|l| {
        l.chars().skip(1).step_by(4).enumerate().for_each(|(i, e)| {
            if e.is_alphabetic() {
                state[i].push(e);
            }
        });
    });

    let instructions: Vec<Instr> = instructions.lines().flat_map(Instr::from_str).collect();

    (state, instructions)
}

fn part1(input: &str) -> String {
    let (mut state, instructions) = parse_input(input);
    for Instr { amount, from, to } in instructions {
        for _ in 0..amount {
            if let Some(popped) = state[from].pop() {
                state[to].push(popped);
            }
        }
    }

    let result = state.iter().flat_map(|v| v.last()).collect::<String>();
    result
}

fn part2(input: &str) -> String {
    let (mut state, instructions) = parse_input(input);
    for Instr { amount, from, to } in instructions {
        let from_idx = state[from].len() - amount;
        let mut elements = state[from].split_off(from_idx);
        state[to].append(&mut elements);
    }

    let result = state.iter().flat_map(|v| v.last()).collect::<String>();
    result
}

fn main() {
    let input = fs::read_to_string("inputs/day05.txt").expect("Could not read input file");
    let part1 = part1(&input);
    let part2 = part2(&input);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3  

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        let msg = part1(TEST_INPUT.trim_start_matches('\n'));
        assert_eq!(msg, "CMZ");
    }
    #[test]
    fn test_part2() {
        let msg = part2(TEST_INPUT.trim_start_matches('\n'));
        assert_eq!(msg, "MCD");
    }
}
