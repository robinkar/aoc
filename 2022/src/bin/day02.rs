use std::fs;

#[derive(Clone, Copy)]
enum RPS {
    Rock = 1,
    Paper,
    Scissor,
}

impl RPS {
    fn wins(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (RPS::Rock, RPS::Scissor) | (RPS::Paper, RPS::Rock) | (RPS::Scissor, RPS::Paper)
        )
    }
    // Returns the shape to choose to win over self
    fn winning(&self) -> Self {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissor,
            RPS::Scissor => RPS::Rock,
        }
    }
    fn from_char(c: char) -> Option<Self> {
        match c {
            'A' | 'X' => Some(Self::Rock),
            'B' | 'Y' => Some(Self::Paper),
            'C' | 'Z' => Some(Self::Scissor),
            _ => None,
        }
    }
}

enum Strategy {
    Win,
    Draw,
    Lose,
}

impl Strategy {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'X' => Some(Self::Lose),
            'Y' => Some(Self::Draw),
            'Z' => Some(Self::Win),
            _ => None,
        }
    }
}

struct Round {
    ply: RPS,
    opp: RPS,
}

impl Round {
    fn from_line_part1(line: &str) -> Option<Self> {
        let (opp, ply) = Self::round_chars(line)?;
        Some(Self {
            ply: RPS::from_char(ply)?,
            opp: RPS::from_char(opp)?,
        })
    }
    fn from_line_part2(line: &str) -> Option<Self> {
        let (opp, strategy) = Self::round_chars(line)?;
        let opp = RPS::from_char(opp)?;
        let strategy = Strategy::from_char(strategy)?;
        let ply = match strategy {
            Strategy::Win => opp.winning(),
            Strategy::Draw => opp,
            Strategy::Lose => opp.winning().winning(),
        };
        Some(Self { ply, opp })
    }
    fn round_chars(line: &str) -> Option<(char, char)> {
        let mut round = line.trim().split(' ').map(str::chars);
        Some((round.next()?.next()?, round.next()?.next()?))
    }
    // Score gained from playing one round
    fn score(self) -> u32 {
        let score = match (self.ply, self.opp) {
            (ply, opp) if ply.wins(&opp) => 6,
            (ply, opp) if opp.wins(&ply) => 0,
            _ => 3,
        };
        score + self.ply as u32
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .flat_map(Round::from_line_part1)
        .map(Round::score)
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .flat_map(Round::from_line_part2)
        .map(Round::score)
        .sum()
}

fn main() {
    let input = fs::read_to_string("inputs/day02.txt").expect("Could not read input file");
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
        A Y
        B X
        C Z"#;
        let score = part1(&input);
        assert_eq!(score, 15);
    }
    #[test]
    fn test_part2() {
        let input = r#"
        A Y
        B X
        C Z"#;
        let score = part2(&input);
        assert_eq!(score, 12);
    }
}
