use std::{fs, ops::RangeInclusive, str::FromStr, time::Duration};

trait RangeExt
where
    Self: Sized,
{
    fn surrounds(&self, other: &Self) -> bool;
    fn intersects_end(&self, other: &Self) -> bool;
    fn combined(&self, other: &Self) -> Option<Self>;
}

impl RangeExt for RangeInclusive<i64> {
    fn surrounds(&self, other: &Self) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }

    fn intersects_end(&self, other: &Self) -> bool {
        self.end() + 1 == *other.start()
            || (self.start() <= other.start() && other.contains(self.end()))
    }

    fn combined(&self, other: &Self) -> Option<Self> {
        if self.surrounds(other) {
            Some(self.clone())
        } else if other.surrounds(self) {
            Some(other.clone())
        } else if self.intersects_end(other) {
            Some(*self.start()..=*other.end())
        } else if other.intersects_end(self) {
            Some(*other.start()..=*self.end())
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
struct Sensor {
    x: i64,
    y: i64,
    range: i64,
}

impl Sensor {
    fn range_on_row(&self, y: i64) -> Option<RangeInclusive<i64>> {
        let dy = self.y.abs_diff(y) as i64;
        let range = self.range - dy;
        if range >= 0 {
            Some((self.x - range)..=self.x + range)
        } else {
            None
        }
    }
}

fn next_num(it: &mut impl Iterator<Item = char>) -> Result<i64, std::num::ParseIntError> {
    it.by_ref()
        .skip_while(|&c| c != '=')
        .skip(1)
        .take_while(|&c| !(c == ',' || c == ':'))
        .collect::<String>()
        .parse::<i64>()
}

impl FromStr for Sensor {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.chars();
        let x = next_num(&mut iter)?;
        let y = next_num(&mut iter)?;
        let cx = next_num(&mut iter)?;
        let cy = next_num(&mut iter)?;
        Ok(Self {
            x,
            y,
            range: (x - cx).abs() + (y - cy).abs(),
        })
    }
}

fn parse_input(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .map(Sensor::from_str)
        .collect::<Result<Vec<Sensor>, _>>()
        .expect("Invalid input")
}

// Inserts range into ranges, combinding ranges if they intersect or overlap
fn insert_range(ranges: &mut Vec<RangeInclusive<i64>>, range: RangeInclusive<i64>) {
    let mut to_replace: Option<usize> = None;

    let mut merged: Option<RangeInclusive<i64>> = None;

    for (idx, r) in ranges.iter_mut().enumerate() {
        if let Some(combined) = r.combined(&range) {
            merged = Some(combined);
            to_replace = Some(idx);
            break;
        }
    }
    match (merged, to_replace) {
        (Some(r), Some(idx)) => {
            ranges.remove(idx);
            insert_range(ranges, r);
        }
        _ => ranges.push(range),
    }
}

fn get_row_ranges(sensors: &[Sensor], row: i64, min: i64, max: i64) -> Vec<RangeInclusive<i64>> {
    sensors.iter().flat_map(|b| b.range_on_row(row)).fold(
        Vec::<RangeInclusive<i64>>::new(),
        |mut ranges, range| {
            insert_range(
                &mut ranges,
                *range.start().max(&min)..=*range.end().min(&max),
            );
            ranges
        },
    )
}

fn part1(input: &str, row: i64) -> i64 {
    let sensors = parse_input(input);
    let len = get_row_ranges(&sensors, row, i32::MIN as i64, i32::MAX as i64)
        .iter()
        .map(|r| r.end() - r.start())
        .sum::<i64>();
    len
}

fn part2(input: &str, max_coord: i64) -> i64 {
    let sensors = parse_input(input);
    let (row, mut ranges) = (0..=max_coord)
        .map(|i| get_row_ranges(&sensors, i, 0, max_coord))
        .enumerate()
        .find(|(_, ranges)| ranges.len() > 1)
        .expect("Beacon not found");
    ranges.sort_by(|a, b| a.start().cmp(b.start()));
    4_000_000 * (ranges[0].end() + 1) + row as i64
}

fn main() {
    let input = fs::read_to_string("inputs/day15.txt").expect("Could not read input file");
    aoc::bench!(part1(&input, 2_000_000), part1, p1);
    aoc::bench!(part2(&input, 4_000_000), part2, p2);
    println!("Part 1: {part1} ({} us)", p1.as_micros());
    println!("Part 2: {part2} ({} us)", p2.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT, 10), 26);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT, 20), 56000011);
    }
}
