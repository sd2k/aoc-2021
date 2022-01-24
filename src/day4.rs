use std::convert::identity;

use anyhow::Result;
use itertools::Itertools;
use serde::Deserialize;

use aoc_runner_derive::{aoc, aoc_generator};

/// The value of a cell.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Value(u8);

#[derive(Clone, Copy, Debug)]
struct Row(u8);

#[derive(Clone, Copy, Debug, PartialEq)]
struct Column(u8);

#[derive(Clone, Copy, Debug)]
enum Winner {
    Row(Row),
    Column(Column),
}

#[derive(Clone, Debug)]
struct Cell {
    row: Row,
    column: Column,
    value: Value,
    drawn: bool,
}

impl Cell {
    fn new(row: Row, column: Column, value: Value) -> Self {
        Self {
            row,
            column,
            value,
            drawn: false,
        }
    }
}

#[derive(Clone, Debug)]
struct Board {
    cells: Vec<Cell>,
}

impl FromIterator<Cell> for Board {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Cell>,
    {
        Self {
            cells: iter.into_iter().collect(),
        }
    }
}

impl Board {
    fn draw(&mut self, v: Value) {
        if let Some(c) = self.cells.iter_mut().find(|c| c.value == v) {
            c.drawn = true
        }
    }

    fn row(&self, i: Row) -> impl Iterator<Item = &Cell> {
        self.cells.iter().skip(i.0 as usize * 5).take(5)
    }

    fn column(&self, j: Column) -> impl Iterator<Item = &Cell> {
        self.cells.iter().filter(move |c| c.column == j)
    }

    fn check(&self) -> Option<Winner> {
        (0..5)
            .find_map(|i| {
                self.row(Row(i))
                    .all(|x| x.drawn)
                    .then(|| Winner::Row(Row(i)))
            })
            .or_else(|| {
                (0..5).find_map(|j| {
                    self.column(Column(j))
                        .all(|x| x.drawn)
                        .then(|| Winner::Column(Column(j)))
                })
            })
    }

    fn sum_of_undrawn_numbers(&self) -> usize {
        self.cells
            .iter()
            .filter_map(|cell| (!cell.drawn).then(|| cell.value.0 as usize))
            .sum()
    }
}

type Input = (Vec<Value>, Vec<Board>);

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| Value(x.parse().unwrap()))
        .collect();
    let boards = lines
        .chunks(6)
        .into_iter()
        .map(|chunk| {
            chunk
                .skip(1)
                .enumerate()
                .flat_map(move |(row, line)| {
                    line.split_whitespace()
                        .enumerate()
                        .map(move |(column, value)| {
                            Cell::new(
                                Row(row as u8),
                                Column(column as u8),
                                Value(value.parse().unwrap()),
                            )
                        })
                })
                .collect()
        })
        .collect();
    (numbers, boards)
}

#[aoc(day4, part1)]
fn part1(input: &Input) -> usize {
    let mut boards: Vec<Board> = input.1.clone();
    input
        .0
        .iter()
        .find_map(|value| {
            boards.iter_mut().find_map(|board| {
                board.draw(*value);
                board
                    .check()
                    .is_some()
                    .then(|| value.0 as usize * board.sum_of_undrawn_numbers())
            })
        })
        .expect("a board to win before we run out of numbers")
    // let values = input.0.iter();
    // loop {
    //     let value = values.next().expect("a board to win");
    //     boards.iter_mut().for_each(|board| board.draw(*value));
    //     let winning_board = boards.iter().find(|board| board.check().is_some());
    //     if let Some(w) = winning_board {
    //         return value.0 as usize * w.sum_of_undrawn_numbers();
    //     }
    // }
}

#[aoc(day4, part2)]
fn part2(input: &Input) -> usize {
    let mut boards: Vec<Board> = input.1.clone();
    input
        .0
        .iter()
        .find_map(|value| {
            boards.iter_mut().for_each(|board| board.draw(*value));
            if boards.len() != 1 {
                boards.retain(|board| !board.check().is_some());
                None
            } else {
                boards[0]
                    .check()
                    .is_some()
                    .then(|| value.0 as usize * boards[0].sum_of_undrawn_numbers())
            }
        })
        .expect("a board to win before we run out of numbers")
    // let mut values = input.0.iter().copied();
    // loop {
    //     let value = values.next().expect("a board to win");
    //     boards.iter_mut().for_each(|board| board.draw(value));
    //     if boards.len() != 1 {
    //         boards.retain(|board| !board.check().is_some());
    //     } else {
    //         if boards[0].check().is_some() {
    //             return value.0 as usize * boards[0].sum_of_undrawn_numbers();
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&parse_input(
                "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"
            )),
            4512
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse_input(
                "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19

     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7"
            )),
            1924
        );
    }
}
