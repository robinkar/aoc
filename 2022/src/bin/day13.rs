use std::{cmp::Ordering, fmt::Display, fs, str::FromStr};

#[derive(Debug, Clone, Eq, PartialEq)]
enum PacketData {
    Int(u32),
    List(Vec<PacketData>),
}

impl Display for PacketData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(n) => write!(f, "{}", n),
            Self::List(v) => write!(
                f,
                "[{}]",
                v.iter()
                    .map(PacketData::to_string)
                    .collect::<Vec<String>>()
                    .join(",")
            ),
        }
    }
}

fn find_matching_bracket(s: &str) -> Option<usize> {
    let mut level = 0;
    for (i, c) in s.char_indices() {
        match c {
            '[' => level += 1,
            ']' if level == 0 => return Some(i),
            ']' => level -= 1,
            _ => {}
        };
    }
    None
}

impl FromStr for PacketData {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<u32>() {
            return Ok(Self::Int(n));
        };
        let mut data: Vec<Self> = vec![];
        let mut inner = s;
        while !inner.is_empty() {
            if inner.starts_with('[') {
                let end_idx = find_matching_bracket(&inner[1..]).expect("Missing end bracket");
                let inner_data = Self::from_str(&inner[1..=end_idx])?;
                let new_data = match inner_data {
                    Self::Int(n) => Self::List(vec![Self::Int(n)]),
                    v => v,
                };
                data.push(new_data);
                inner = match &inner.get(end_idx + 3..) {
                    Some(i) => i,
                    None => break,
                }
            } else {
                let next_comma = inner.find(|c| c == ',').unwrap_or(inner.len());
                let num = inner[..next_comma].parse::<u32>().expect("Invalid number");
                data.push(Self::Int(num));
                inner = match &inner.get(next_comma + 1..) {
                    Some(i) => i,
                    None => break,
                };
            }
        }
        Ok(Self::List(data))
    }
}

impl Ord for PacketData {
    fn cmp(&self, right: &Self) -> Ordering {
        match (self, right) {
            (Self::Int(l), Self::Int(r)) => l.cmp(r),
            (Self::List(l), Self::List(r)) => l.cmp(r),
            (Self::Int(l), Self::List(r)) => [Self::Int(*l)][..].cmp(r),
            (Self::List(l), Self::Int(r)) => l.as_slice().cmp(&[Self::Int(*r)]),
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> Vec<(PacketData, PacketData)> {
    input
        .split("\n\n")
        .map(|pair| pair.split_once('\n').expect("Missing pair"))
        .map(|(l, r)| {
            (
                PacketData::from_str(l).expect("Invalid input in left"),
                PacketData::from_str(r).expect("Invalid input in right"),
            )
        })
        .collect()
}

fn part1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .enumerate()
        .map(|(i, (l, r))| if l <= r { i + 1 } else { 0 })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut packets = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| PacketData::from_str(line).expect("Invalid input"))
        .collect::<Vec<PacketData>>();
    let d1 = PacketData::from_str("[[2]]").unwrap();
    let d2 = PacketData::from_str("[[6]]").unwrap();
    packets.push(d1.clone());
    packets.push(d2.clone());
    packets.sort();

    let idx1 = packets
        .iter()
        .position(|p| p == &d1)
        .expect("Divider packet 1 not found");
    let idx2 = packets
        .iter()
        .position(|p| p == &d2)
        .expect("Divider packet 2 not found");

    (idx1 + 1) * (idx2 + 1)
}

fn main() {
    let input = fs::read_to_string("inputs/day13.txt").expect("Could not read input file");
    let part1 = part1(&input);
    let part2 = part2(&input);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[10]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 13);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 140);
    }

    #[test]
    fn test_parsing() {
        let parsed = parse_input(TEST_INPUT)
            .iter()
            .map(|(l, r)| (l.to_string(), r.to_string()))
            .collect::<Vec<(String, String)>>();
        let pairs = TEST_INPUT
            .split("\n\n")
            .map(|pair| pair.split_once('\n').unwrap())
            .collect::<Vec<(&str, &str)>>();
        for ((l_parsed, r_parsed), &(l_real, r_real)) in parsed.iter().zip(pairs.iter()) {
            assert_eq!(&l_parsed[1..l_parsed.len() - 1], l_real);
            assert_eq!(&r_parsed[1..r_parsed.len() - 1], r_real);
        }
    }
}
