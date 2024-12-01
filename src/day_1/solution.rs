use std::collections::{BinaryHeap, HashMap};

#[allow(dead_code)]
const SHORT_INPUT: &str = "2   5
1   3
3   9
3   3";

#[allow(dead_code)]
const SHORT_INPUT_TWO: &str = "3   4
4   3
2   5
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
fn part_two(input: &str) -> u32 {
    let mut left_keys = Vec::new();
    let mut key_count: HashMap<u32, u32> = HashMap::new();

    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .for_each(|line| {
            let (left_key, right_key) = split_keys(line);

            left_keys.push(left_key);

            key_count
                .entry(right_key)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

    left_keys.iter().fold(0, |total_similarity, left_key| {
        total_similarity + left_key * key_count.get(left_key).unwrap_or(&0)
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
    use crate::day_1::solution::*;

    #[test]
    fn part_one_short_input() {
        assert_eq!(part_one(SHORT_INPUT), 11);
    }

    #[test]
    fn part_one_puzzle_input() {
        assert_eq!(part_one(PUZZLE_INPUT), 2164381);
    }

    #[test]
    fn part_two_short_input() {
        assert_eq!(part_two(SHORT_INPUT_TWO), 31);
    }

    #[test]
    fn part_two_puzzle_input() {
        assert_eq!(part_two(PUZZLE_INPUT), 20719933);
    }
}
