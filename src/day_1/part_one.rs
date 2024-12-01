use std::collections::BinaryHeap;

#[allow(dead_code)]
const SHORT_INPUT: &str = "2   5
1   3
3   9
3   3";

#[allow(dead_code)]
const PUZZLE_INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
fn part_one(input: &str) -> u32 {
    let mut left_keys = BinaryHeap::new();
    let mut right_keys = BinaryHeap::new();
    let lines = input.lines().filter(|line| !line.trim().is_empty());

    for line in lines {
        let (left_key, right_key) = split_keys(line);

        left_keys.push(left_key);
        right_keys.push(right_key);
    }

    left_keys
        .into_sorted_vec()
        .iter()
        .zip(right_keys.into_sorted_vec().iter())
        .fold(0, |total_distance, (left, right)| {
            total_distance + left.abs_diff(*right)
        })
}

#[allow(dead_code)]
fn split_keys(keys: &str) -> (u32, u32) {
    let mut split = keys.split_whitespace();
    let left = split.next().unwrap().parse().unwrap();
    let right = split.next().unwrap().parse().unwrap();
    (left, right)
}

#[cfg(test)]
mod tests {
    use crate::day_1::part_one::{part_one, PUZZLE_INPUT, SHORT_INPUT};

    #[test]
    fn short_input() {
        assert_eq!(part_one(SHORT_INPUT), 11);
    }

    #[test]
    fn puzzle_input() {
        assert_eq!(part_one(PUZZLE_INPUT), 2164381);
    }
}
