use std::{cmp::Ordering, fmt::Display, fs, ops::RangeInclusive};

#[derive(Clone, Debug, PartialEq)]
enum Tile {
    Empty,
    Rock,
    Sand,
    SandSource,
}

impl Tile {
    fn symbol(&self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Rock => '#',
            Tile::Sand => 'o',
            Tile::SandSource => '+',
        }
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

fn range(start: usize, end: usize) -> RangeInclusive<usize> {
    match start.cmp(&end) {
        Ordering::Less => start..=end,
        Ordering::Equal => start..=start,
        Ordering::Greater => end..=start,
    }
}

fn build_grid(
    coords: &[Vec<(usize, usize)>],
    dim: (usize, usize),
    start_x: usize,
    part2: bool,
) -> Vec<Vec<Tile>> {
    let dim_x = if part2 { dim.0 + 2 * dim.1 } else { dim.0 };
    let dim_y = if part2 { dim.1 + 2 } else { dim.1 };
    let mut grid: Vec<Vec<Tile>> = vec![vec![Tile::Empty; dim_x]; dim_y];

    let start_x = if part2 { start_x - dim_y } else { start_x };
    for line in coords.iter() {
        for segment in line.windows(2) {
            let (sx, sy) = segment[0];
            let (ex, ey) = segment[1];
            for j in range(sy, ey) {
                for i in range(sx, ex) {
                    grid[j][i - start_x] = Tile::Rock;
                }
            }
        }
    }
    if part2 {
        for i in 0..dim_x {
            grid[dim_y - 1][i] = Tile::Rock;
        }
    }
    grid[0][500 - start_x] = Tile::SandSource;
    grid
}

fn parse_input(input: &str, part2: bool) -> Vec<Vec<Tile>> {
    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    let scan = input
        .lines()
        .map(|line| {
            line.split("->")
                .map(|coords| {
                    let (l, r) = coords.trim().split_once(',').expect("Missing comma");
                    let x = l.parse::<usize>().expect("Invalid x coord");
                    let y = r.parse::<usize>().expect("Invalid y coord");
                    min_x = min_x.min(x);
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                    (x, y)
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<Vec<(usize, usize)>>>();
    build_grid(&scan, (max_x - min_x + 1, max_y + 1), min_x, part2)
}

fn tile_empty(grid: &[Vec<Tile>], x: usize, y: usize) -> bool {
    let tile = &grid[y][x];
    tile == &Tile::Empty
}

fn in_grid(grid: &[Vec<Tile>], x: usize, y: usize) -> bool {
    x > 0 && x < grid[0].len() && y < grid.len()
}

fn drop_sand(grid: &[Vec<Tile>], source_x: usize) -> Option<(usize, usize)> {
    let mut x = source_x as i32;
    let mut y = 0i32;
    let possible_offsets = [[0, 1], [-1, 1], [1, 1]];

    'outer: while in_grid(grid, x as usize, y as usize) {
        for [dx, dy] in possible_offsets {
            if tile_empty(grid, (x + dx) as usize, (y + dy) as usize) {
                x += dx;
                y += dy;
                continue 'outer;
            }
        }
        break;
    }
    if in_grid(grid, x as usize, y as usize) && tile_empty(grid, x as usize, y as usize) {
        Some((x as usize, y as usize))
    } else {
        None
    }
}

fn run_simulation(grid: &mut [Vec<Tile>]) -> usize {
    let sand_source = grid[0]
        .iter()
        .position(|t| t == &Tile::SandSource)
        .expect("No sand source in grid");
    while let Some((x, y)) = drop_sand(grid, sand_source) {
        grid[y][x] = Tile::Sand;
    }
    grid.iter()
        .map(|row| row.iter().filter(|&t| t == &Tile::Sand).count())
        .sum()
}

fn part1(input: &str) -> usize {
    let mut grid = parse_input(input, false);
    run_simulation(&mut grid)
}

fn part2(input: &str) -> usize {
    let mut grid = parse_input(input, true);
    run_simulation(&mut grid) + 1
}

fn main() {
    let input = fs::read_to_string("inputs/day14.txt").expect("Could not read input file");
    let part1 = part1(&input);
    let part2 = part2(&input);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 24);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 93);
    }
}
