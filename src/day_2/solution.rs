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
}
