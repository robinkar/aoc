use std::{cmp::Ordering, collections::BinaryHeap, convert::TryInto, fs};

type Pos = (usize, usize);

#[derive(Copy, Clone, Eq, Debug, PartialEq)]
struct Node {
    pos: Pos,
    height: i32,
    g: i32,
    h: i32,
    parent: Option<Pos>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f().cmp(&self.f())
    }
}

impl Node {
    fn f(&self) -> i32 {
        self.g + self.h
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str, multiple_starts: bool) -> (Vec<Vec<Node>>, Vec<Pos>, Pos) {
    let mut starts: Vec<Pos> = vec![];
    let mut end: Option<Pos> = None;

    let mut nodes = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(x, &b)| {
                    if b == b'S' || (multiple_starts && b == b'a') {
                        starts.push((x, y));
                    } else if b == b'E' {
                        end = Some((x, y));
                    }
                    Node {
                        pos: (x, y),
                        height: (b.saturating_sub(b'a')) as i32,
                        parent: None,
                        g: i32::MAX - 1,
                        h: i32::MAX - 1,
                    }
                })
                .collect::<Vec<Node>>()
        })
        .collect::<Vec<Vec<Node>>>();

    let end = end.expect("End not found");
    nodes[end.1][end.0].height = (b'z' - b'a') as i32;

    (nodes, starts, end)
}

fn heuristic(pos: Pos, end: Pos) -> i32 {
    (((end.0 as isize - pos.0 as isize).pow(2) + (end.1 as isize - pos.1 as isize).pow(2)) as f32)
        .sqrt() as i32
}

fn neighbours(pos: Pos) -> Vec<Pos> {
    [(1, 0), (0, 1), (-1, 0), (0, -1)]
        .iter()
        .flat_map(|&(dx, dy)| {
            Some((
                (pos.0 as isize).checked_add(dx)?.try_into().ok()?,
                (pos.1 as isize).checked_add(dy)?.try_into().ok()?,
            ))
        })
        .collect()
}

// Length of shortest path from start to end
fn a_star(nodes: &[Vec<Node>], start: Pos, end: Pos) -> Option<u32> {
    let mut nodes = nodes.to_vec();
    let mut open_set = BinaryHeap::<Node>::new();
    nodes[start.1][start.0].g = 0;
    open_set.push(nodes[start.1][start.0]);

    while let Some(node) = open_set.pop() {
        if node.pos == end {
            break;
        }
        for (i, j) in neighbours(node.pos) {
            if let Some(&neighbour) = nodes.get(j).and_then(|row| row.get(i)) {
                if neighbour.height - node.height > 1 {
                    continue;
                }
                if node.g + 1 < neighbour.g {
                    let nb = Node {
                        parent: Some(node.pos),
                        g: node.g + 1,
                        h: heuristic(neighbour.pos, end),
                        ..neighbour
                    };
                    open_set.push(nb);
                    nodes[neighbour.pos.1][neighbour.pos.0] = nb;
                }
            }
        }
    }

    let (mut x, mut y) = end;
    let mut len = 0u32;
    while nodes[y][x].pos != start {
        (x, y) = nodes[y][x].parent?;
        len += 1;
    }
    Some(len)
}

fn part1(input: &str) -> u32 {
    let (nodes, starts, end) = parse_input(input, false);
    let start = starts[0];
    let path = a_star(&nodes, start, end);
    path.expect("No path found")
}

fn part2(input: &str) -> u32 {
    let (nodes, starts, end) = parse_input(input, true);
    starts
        .iter()
        .flat_map(|&start| {
            let nodes = nodes.clone();
            a_star(&nodes, start, end)
        })
        .min()
        .expect("No path found")
}

fn main() {
    let input = fs::read_to_string("inputs/day12.txt").expect("Could not read input file");
    let part1 = part1(&input);
    let part2 = part2(&input);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 31);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 29);
    }
}
