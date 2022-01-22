use aoc_runner_derive::{aoc, aoc_generator};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum Direction {
    Up,
    Down,
    Forward,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Command {
    direction: Direction,
    magnitude: i32,
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| serde_scan::from_str(line).unwrap())
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Command]) -> i32 {
    let (horizontal, depth) =
        input
            .iter()
            .fold((0i32, 0i32), |acc, command| match command.direction {
                Direction::Up => (acc.0, acc.1 - command.magnitude),
                Direction::Down => (acc.0, acc.1 + command.magnitude),
                Direction::Forward => (acc.0 + command.magnitude, acc.1),
            });
    horizontal * depth
}

#[aoc(day2, part2)]
fn part2(input: &[Command]) -> i32 {
    let (horizontal, depth, _) = input.iter().fold((0i32, 0i32, 0i32), |(h, d, a), command| {
        match command.direction {
            Direction::Up => (h, d, a - command.magnitude),
            Direction::Down => (h, d, a + command.magnitude),
            Direction::Forward => (h + command.magnitude, d + (command.magnitude * a), a),
        }
    });
    horizontal * depth
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(
            parse_input(input),
            vec![
                Command {
                    direction: Direction::Forward,
                    magnitude: 5
                },
                Command {
                    direction: Direction::Down,
                    magnitude: 5
                },
                Command {
                    direction: Direction::Forward,
                    magnitude: 8
                },
                Command {
                    direction: Direction::Up,
                    magnitude: 3
                },
                Command {
                    direction: Direction::Down,
                    magnitude: 8
                },
                Command {
                    direction: Direction::Forward,
                    magnitude: 2
                },
            ]
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&[
                Command {
                    direction: Direction::Forward,
                    magnitude: 5
                },
                Command {
                    direction: Direction::Down,
                    magnitude: 5
                },
                Command {
                    direction: Direction::Forward,
                    magnitude: 8
                },
                Command {
                    direction: Direction::Up,
                    magnitude: 3
                },
                Command {
                    direction: Direction::Down,
                    magnitude: 8
                },
                Command {
                    direction: Direction::Forward,
                    magnitude: 2
                },
            ]),
            150
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&[
                Command {
                    direction: Direction::Forward,
                    magnitude: 5
                },
                Command {
                    direction: Direction::Down,
                    magnitude: 5
                },
                Command {
                    direction: Direction::Forward,
                    magnitude: 8
                },
                Command {
                    direction: Direction::Up,
                    magnitude: 3
                },
                Command {
                    direction: Direction::Down,
                    magnitude: 8
                },
                Command {
                    direction: Direction::Forward,
                    magnitude: 2
                },
            ]),
            900
        );
    }
}
