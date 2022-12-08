use std::{
    collections::{HashMap, HashSet},
    fs,
    hash::Hash,
};

fn transpose<T: Clone>(vector: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = vector.first().unwrap().len();
    vector
        .into_iter()
        .fold(vec![Vec::<T>::new(); len], |mut acc, e| {
            acc.iter_mut()
                .zip(e.into_iter())
                .for_each(|(v, c)| v.push(c));
            acc
        })
        .into_iter()
        .map(|v| v.into_iter().collect::<Vec<T>>())
        .collect()
}

fn parse_input<T: From<u8>>(input: &str) -> Vec<Vec<T>> {
    input
        .lines()
        .map(str::as_bytes)
        .map(|line| {
            line.iter()
                .map(|c| T::from(*c - b'0'))
                .collect::<Vec<T>>()
        })
        .collect::<Vec<Vec<T>>>()
}

fn maybe_insert_and_update_max<T: Eq + Hash, U: PartialOrd<U>>(
    set: &mut HashSet<T>,
    max: &mut U,
    val: U,
    to_insert: T,
) {
    if &val > max {
        *max = val;
        set.insert(to_insert);
    }
}

fn part1(input: &str) -> usize {
    let parsed = parse_input::<i16>(input);
    let mut visible = HashSet::<(usize, usize)>::new();

    for (j, row) in parsed.iter().enumerate() {
        let mut max = -1i16;
        let mut max_rev = -1i16;
        let len = row.len() - 1;
        // Right
        for (i, tree) in row.iter().enumerate() {
            maybe_insert_and_update_max(&mut visible, &mut max, *tree, (i, j));
        }
        // Left
        for (i, tree) in row.iter().rev().enumerate() {
            maybe_insert_and_update_max(&mut visible, &mut max_rev, *tree, (len - i, j));
        }
    }
    let parsed = transpose(parsed);
    for (i, col) in parsed.iter().enumerate() {
        let mut max = -1i16;
        let mut max_rev = -1i16;
        let len = col.len() - 1;
        // Down
        for (j, tree) in col.iter().enumerate() {
            maybe_insert_and_update_max(&mut visible, &mut max, *tree, (i, j));
        }
        // Up
        for (j, tree) in col.iter().rev().enumerate() {
            maybe_insert_and_update_max(&mut visible, &mut max_rev, *tree, (i, len - j));
        }
    }
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

fn part2(input: &str) -> usize {
    let parsed = parse_input::<usize>(input);
    let mut trees = HashMap::<(usize, usize), usize>::new();

    let mut scores = vec![0usize; 10];
    let mut scores_rev = vec![0usize; 10];

    for (j, row) in parsed.iter().enumerate() {
        let len = row.len() - 1;
        scores.splice(0..10, vec![0; 10]);
        scores_rev.splice(0..10, vec![0; 10]);
        // Right
        for (i, tree) in row.iter().enumerate() {
            let score = update_scores(&mut scores, *tree);
            *trees.entry((i, j)).or_insert(1) *= score;
        }
        // Left
        for (i, tree) in row.iter().rev().enumerate() {
            let i = len - i;
            let score = update_scores(&mut scores_rev, *tree);
            *trees.entry((i, j)).or_insert(1) *= score;
        }
    }

    let parsed = transpose(parsed);
    for (i, col) in parsed.iter().enumerate() {
        let len = col.len() - 1;
        scores.splice(0..10, vec![0; 10]);
        scores_rev.splice(0..10, vec![0; 10]);
        // Down
        for (j, tree) in col.iter().enumerate() {
            let score = update_scores(&mut scores, *tree);
            *trees.entry((i, j)).or_insert(1) *= score;
        }
        // Up
        for (j, tree) in col.iter().rev().enumerate() {
            let j = len - j;
            let score = update_scores(&mut scores_rev, *tree);
            *trees.entry((i, j)).or_insert(1) *= score;
        }
    }
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
