use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Crab(u16);

type Input = Vec<Crab>;

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Input {
    input.split(',').map(|i| Crab(i.parse().unwrap())).collect()
}

fn best_position_part1(crabs: &Input) -> u16 {
    let mut crabs = crabs.clone();
    crabs.sort();
    crabs[crabs.len() / 2].0
}

fn best_position_part2(crabs: &Input) -> (u16, u16) {
    let mean = crabs.iter().map(|crab| crab.0 as u32).sum::<u32>() as f32 / crabs.len() as f32;
    (mean.floor() as u16, mean.ceil() as u16)
}

fn cost_part1(crabs: &Input, position: u16) -> usize {
    crabs
        .iter()
        .map(|crab| (crab.0 as isize - position as isize).abs() as usize)
        .sum()
}

fn sum_1_to_n(n: usize) -> usize {
    (n * (n + 1)) / 2
}

fn cost_part2(crabs: &Input, position: u16) -> usize {
    crabs
        .iter()
        .map(|crab| sum_1_to_n((crab.0 as isize - position as isize).abs() as usize))
        .sum()
}

#[aoc(day7, part1)]
fn part1(input: &Input) -> usize {
    cost_part1(&input, best_position_part1(&input))
}

#[aoc(day7, part2)]
fn part2(input: &Input) -> usize {
    let candidates = best_position_part2(&input);
    [
        cost_part2(&input, candidates.0),
        cost_part2(&input, candidates.1),
    ]
    .into_iter()
    .min()
    .unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input("16,1,2,0,4,2,7,1,2,14")), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input("16,1,2,0,4,2,7,1,2,14")), 168);
    }
}
