use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::hash_map::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Fish(u8);

type Input = HashMap<Fish, usize>;

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Input {
    input
        .split(',')
        .map(|i| Fish(i.parse().unwrap()))
        .fold(HashMap::new(), |mut acc, x| {
            acc.entry(x).and_modify(|y| *y += 1).or_insert(1);
            acc
        })
}

fn evolve(fish: &Input) -> Input {
    fish.iter()
        .flat_map(|(k, v)| -> Vec<(Fish, usize)> {
            if k == &Fish(0) {
                vec![(Fish(6), *v), (Fish(8), *v)]
            } else {
                vec![(Fish(k.0 - 1), *v)]
            }
        })
        .fold(HashMap::new(), |mut acc, (fish, v)| {
            acc.entry(fish).and_modify(|x| *x += v).or_insert(v);
            acc
        })
}

#[aoc(day6, part1)]
fn part1(input: &Input) -> usize {
    let mut fish = input.clone();
    for _ in 0..80 {
        fish = evolve(&fish);
    }
    fish.values().sum()
}

#[aoc(day6, part2)]
fn part2(input: &Input) -> usize {
    let mut fish = input.clone();
    for _ in 0..256 {
        fish = evolve(&fish);
    }
    fish.values().sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input("3,4,3,1,2")), 5934);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input("3,4,3,1,2")), 26984457539);
    }
}
