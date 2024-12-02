#[allow(dead_code)]
const SHORT_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

#[allow(dead_code)]
const PUZZLE_INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const PART_TWO_EDGE_CASES: &str = "48 46 47 49 51 54 56
1 1 2 3 4 5
1 2 3 4 5 5
5 1 2 3 4 5
1 4 3 2 1
1 6 7 8 9
1 2 3 4 3
9 8 7 6 7
7 10 8 10 11
29 28 27 25 26 25 22 20
1 1 1 1
8 9 10 11
1 2 3 4 5 5
17 11 9 9 7";

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
enum Change {
    Increase,
    Decrease,
}

#[allow(dead_code)]
fn part_one(input: &str) -> u32 {
    input.lines().fold(0, |mut safe_reports, current_line| {
        if current_line
            .split_whitespace()
            .fold(
                (true, None, None),
                |(is_safe, previous_level, mut change), current_level| {
                    if !is_safe {
                        return (is_safe, None, None);
                    }

                    let current_level = current_level.parse::<u32>().unwrap();

                    if previous_level.is_none() {
                        return (is_safe, Some(current_level), change);
                    }

                    if current_level == previous_level.unwrap() {
                        return (false, None, None);
                    }

                    if current_level < previous_level.unwrap() {
                        if change
                            .as_ref()
                            .is_some_and(|c| matches!(c, Change::Increase))
                        {
                            return (false, None, None);
                        }

                        change = Some(Change::Decrease);
                    }

                    if current_level > previous_level.unwrap() {
                        if change
                            .as_ref()
                            .is_some_and(|c| matches!(c, Change::Decrease))
                        {
                            return (false, None, None);
                        }

                        change = Some(Change::Increase);
                    }

                    if !(1..=3).contains(&current_level.abs_diff(previous_level.unwrap())) {
                        return (false, None, None);
                    }

                    (is_safe, Some(current_level), change)
                },
            )
            .0
        {
            safe_reports += 1;
        }

        safe_reports
    })
}

#[allow(dead_code)]
fn part_two(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| {
            let levels: Vec<u32> = line
                .split_whitespace()
                .map(|l| l.parse().unwrap())
                .collect();

            is_safe(&levels)
                || (levels.len() > 1
                    && (0..levels.len()).any(|i| is_safe(&remove_level(&levels, i))))
        })
        .count() as u32
}

fn is_safe(levels: &[u32]) -> bool {
    let mut change = None;
    let mut previous_level = None;

    levels.windows(2).all(|window| {
        let (prev, current) = (window[0], window[1]);

        if (1..=3).contains(&(current.abs_diff(prev))) {
            let is_increasing = current > prev;

            if change == Some(!is_increasing) {
                return false;
            }

            change = Some(is_increasing);
            previous_level = Some(current);

            true
        } else {
            false
        }
    })
}

fn remove_level(levels: &[u32], index: usize) -> Vec<u32> {
    let mut new_levels = levels.to_vec();

    new_levels.remove(index);

    new_levels
}

#[cfg(test)]
mod tests {
    use crate::day_2::solution::*;

    #[test]
    fn part_one_short_input() {
        assert_eq!(part_one(SHORT_INPUT), 2);
    }

    #[test]
    fn part_one_puzzle_input() {
        assert_eq!(part_one(PUZZLE_INPUT), 510);
    }

    #[test]
    fn part_two_short_input() {
        assert_eq!(part_two(SHORT_INPUT), 4);
    }

    #[test]
    fn part_two_puzzle_input() {
        assert_eq!(part_two(PUZZLE_INPUT), 553);
    }

    #[test]
    fn part_two_edge_cases() {
        assert_eq!(part_two(PART_TWO_EDGE_CASES), 12);
    }
}
