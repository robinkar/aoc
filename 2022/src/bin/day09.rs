use std::{collections::HashSet, fs, ops, str::FromStr};

#[derive(Clone, Eq, Hash, PartialEq)]
struct Pos(i32, i32);

impl ops::Add<&Pos> for &Pos {
    type Output = Pos;
    fn add(self, rhs: &Pos) -> Pos {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Sub<&Pos> for &Pos {
    type Output = Pos;
    fn sub(self, rhs: &Pos) -> Pos {
        Pos(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::Add<&Dir> for &Pos {
    type Output = Pos;
    fn add(self, rhs: &Dir) -> Pos {
        match rhs {
            Dir::R => Pos(self.0 + 1, self.1),
            Dir::L => Pos(self.0 - 1, self.1),
            Dir::D => Pos(self.0, self.1 + 1),
            Dir::U => Pos(self.0, self.1 - 1),
        }
    }
}

enum Dir {
    R,
    L,
    D,
    U,
}

impl FromStr for Dir {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "R" => Dir::R,
            "L" => Dir::L,
            "D" => Dir::D,
            "U" => Dir::U,
            _ => Err("Invalid direction")?,
        };
        Ok(result)
    }
}

fn parse_input(input: &str) -> Vec<(Dir, u32)> {
    input
        .lines()
        .flat_map(|line| {
            line.split_once(' ')
                .and_then(|(c, n)| Some((c.parse::<Dir>().ok()?, n.parse::<u32>().ok()?)))
        })
        .collect()
}

fn new_pos(head: &Pos, tail: &Pos, dir: &Dir) -> (Pos, Pos) {
    let new_head = head + dir;
    let new_tail = tail_pos_to_head(head, tail);
    (new_head, new_tail)
}

fn tail_pos_to_head(head: &Pos, tail: &Pos) -> Pos {
    let Pos(dx, dy) = head - tail;
    if dx.abs() + dy.abs() == 1 || dx.abs() == 1 && dy.abs() == 1 {
        tail.clone()
    } else {
        tail + &Pos(dx.min(1).max(-1), dy.min(1).max(-1))
    }
}

fn part1(input: &str) -> usize {
    let parsed = parse_input(input);
    let mut tail_positions = HashSet::<Pos>::new();
    let mut head = Pos(0, 0);
    let mut tail = Pos(0, 0);
    for (dir, n) in parsed.iter() {
        for _ in 0..*n {
            (head, tail) = new_pos(&head, &tail, dir);
            tail_positions.insert(tail.clone());
        }
    }
    tail_positions.len()
}

fn part2(input: &str) -> usize {
    let parsed = parse_input(input);
    let mut tail_positions = HashSet::<Pos>::new();
    let mut rope: Vec<Pos> = vec![Pos(0, 0); 10];

    for (dir, n) in parsed.iter() {
        for _ in 0..*n {
            let (head, _) = new_pos(&rope[0], &rope[1], dir);
            rope[0] = head;
            for i in 0..rope.len() - 1 {
                let tail = tail_pos_to_head(&rope[i], &rope[i + 1]);
                rope[i + 1] = tail;
            }
            tail_positions.insert(rope[9].clone());
        }
    }
    tail_positions.len()
}

fn main() {
    let input = fs::read_to_string("inputs/day09.txt").expect("Could not read input file");
    let part1 = part1(&input);
    let part2 = part2(&input);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 13);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 1);
    }
}
