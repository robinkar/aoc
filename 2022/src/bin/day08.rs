use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn parse_input<T: From<u8>>(input: &str) -> (Vec<T>, usize) {
    let x_len = input.lines().next().expect("Invalid input").len();
    let vec = input
        .lines()
        .flat_map(str::as_bytes)
        .map(|c| T::from(c - b'0'))
        .collect::<Vec<T>>();
    (vec, x_len)
}

fn row_iter_indexed<T>(
    vec: &[T],
    row_length: usize,
) -> impl Iterator<Item = impl Iterator<Item = ((usize, usize), &T)> + DoubleEndedIterator> + Clone
{
    vec.chunks(row_length)
        .enumerate()
        .map(|(j, row)| row.iter().enumerate().map(move |(i, v)| ((i, j), v)))
}

fn column_iter_indexed<T>(
    vec: &[T],
    row_length: usize,
) -> impl Iterator<Item = impl Iterator<Item = ((usize, usize), &T)> + DoubleEndedIterator> + Clone
{
    (0..row_length).map(move |i| {
        vec.iter()
            .skip(i)
            .step_by(row_length)
            .enumerate()
            .map(move |(j, v)| ((i, j), v))
    })
}

fn find_visible<'a>(
    row: impl Iterator<Item = ((usize, usize), &'a i16)> + 'a,
) -> Vec<(usize, usize)> {
    let mut visible = Vec::<(usize, usize)>::new();
    let mut max = -1;
    for ((i, j), tree) in row {
        if tree > &max {
            max = *tree;
            visible.push((i, j));
        }
    }
    visible
}

fn part1(input: &str) -> usize {
    let (parsed, x_len) = parse_input::<i16>(input);
    let mut visible = HashSet::<(usize, usize)>::new();

    let rows = row_iter_indexed(&parsed, x_len);
    let columns = column_iter_indexed(&parsed, x_len);

    let right = rows.clone().flat_map(find_visible);
    let left = rows.map(Iterator::rev).flat_map(find_visible);
    let down = columns.clone().flat_map(find_visible);
    let up = columns.map(Iterator::rev).flat_map(find_visible);

    visible.extend(right);
    visible.extend(left);
    visible.extend(up);
    visible.extend(down);

    visible.len()
}

fn update_scores(scores: &mut [usize], tree_size: usize) -> usize {
    // Updates the accumulated scores and returns the score for the tree
    let score = scores[tree_size];
    scores
        .iter_mut()
        .enumerate()
        .for_each(|(i, s)| if i <= tree_size { *s = 1 } else { *s += 1 });
    score
}

fn find_scores<'a>(
    row: impl Iterator<Item = ((usize, usize), &'a usize)> + 'a,
) -> Vec<((usize, usize), usize)> {
    let mut scores_acc = vec![0usize; 10];
    let mut scores = Vec::<((usize, usize), usize)>::new();
    for (coords, tree) in row {
        let score = update_scores(&mut scores_acc, *tree);
        scores.push((coords, score));
    }
    scores
}

fn part2(input: &str) -> usize {
    let (parsed, x_len) = parse_input::<usize>(input);
    let mut trees = HashMap::<(usize, usize), usize>::new();

    let rows = row_iter_indexed(&parsed, x_len);
    let columns = column_iter_indexed(&parsed, x_len);

    //Right
    rows.clone()
        .flat_map(find_scores)
        .for_each(|(k, score)| *trees.entry(k).or_insert(1) *= score);
    // Left
    rows.map(Iterator::rev)
        .flat_map(find_scores)
        .for_each(|(k, score)| *trees.entry(k).or_insert(1) *= score);
    //Down
    columns
        .clone()
        .flat_map(find_scores)
        .for_each(|(k, score)| *trees.entry(k).or_insert(1) *= score);
    // Up
    columns
        .map(Iterator::rev)
        .flat_map(find_scores)
        .for_each(|(k, score)| *trees.entry(k).or_insert(1) *= score);

    *trees.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1
}

fn main() {
    let input = fs::read_to_string("inputs/day08.txt").expect("Could not read input file");
    let part1 = part1(&input);
    let part2 = part2(&input);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 21);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 8);
    }
}
