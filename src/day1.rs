use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|el| el.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> u32 {
    input
        .iter()
        .fold((0, 0), |acc, item| {
            item.gt(&acc.1)
                .then(|| (acc.0 + 1, *item))
                .unwrap_or((acc.0, *item))
        })
        .0
        - 1
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> u32 {
    input
        .iter()
        .tuple_windows::<(_, _, _)>()
        .fold((0, 0), |acc, item| {
            let sum = item.0 + item.1 + item.2;
            sum.gt(&acc.1)
                .then(|| (acc.0 + 1, sum))
                .unwrap_or((acc.0, sum))
        })
        .0
        - 1
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            7
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            5
        );
    }
}
