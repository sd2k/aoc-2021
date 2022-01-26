use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use serde::Deserialize;

/// Annoying workaround for ranges not going backwards...
fn range_inclusive(a: usize, b: usize) -> Box<dyn Iterator<Item = usize>> {
    if b > a {
        Box::new(a..=b)
    } else {
        Box::new((b..=a).rev())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug, Deserialize)]
struct Vector {
    a: Point,
    b: Point,
}

impl Vector {
    fn is_horizontal(&self) -> bool {
        self.a.x == self.b.x
    }

    fn is_vertical(&self) -> bool {
        self.a.y == self.b.y
    }

    fn points(&self) -> Box<dyn Iterator<Item = Point> + '_> {
        if self.is_horizontal() {
            Box::new(range_inclusive(self.a.y, self.b.y).map(move |y| Point::new(self.a.x, y)))
        } else {
            Box::new(range_inclusive(self.a.x, self.b.x).map(move |x| Point::new(x, self.a.y)))
        }
    }

    fn is_diagonal(&self) -> bool {
        (self.a.x as isize - self.b.x as isize).abs()
            == (self.a.y as isize - self.b.y as isize).abs()
    }

    fn diagonal_points(&self) -> Box<dyn Iterator<Item = Point> + '_> {
        Box::new(
            range_inclusive(self.a.x, self.b.x)
                .zip(range_inclusive(self.a.y, self.b.y))
                .map(move |(x, y)| Point { x, y }),
        )
    }
}

type Input = Vec<Vector>;

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| serde_scan::scan!("{},{} -> {},{}" <- line).unwrap())
        .collect()
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> usize {
    input
        .iter()
        .filter(|v| v.is_horizontal() | v.is_vertical())
        .flat_map(|v| v.points())
        .duplicates()
        .count()
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> usize {
    input
        .iter()
        .filter(|v| v.is_horizontal() || v.is_vertical())
        .flat_map(|v| v.points())
        .chain(
            input
                .iter()
                .filter(|v| v.is_diagonal())
                .flat_map(|v| v.diagonal_points()),
        )
        .duplicates()
        .count()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&parse_input(
                "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
            )),
            5
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse_input(
                "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
            )),
            12
        );
    }
}
