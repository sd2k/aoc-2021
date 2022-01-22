use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse_input(input: &str) -> (u32, Vec<u32>) {
    let numbers = input
        .lines()
        .map(|x| u32::from_str_radix(x, 2).unwrap())
        .collect();
    (input.lines().next().unwrap().len() as u32, numbers)
}

fn is_bit_set(number: u32, which: u32) -> bool {
    number & (1 << (which - 1)) != 0
}

fn more_bits_set(input: &[u32], which: u32) -> bool {
    input
        .iter()
        .map(|x| is_bit_set(*x, which) as u32)
        .sum::<u32>() as f32
        >= (input.len() as f32 / 2.)
}

#[aoc(day3, part1)]
fn part1((max_length, input): &(u32, Vec<u32>)) -> u32 {
    let gamma: u32 = (1..(max_length + 1))
        .map(|k| more_bits_set(input, k))
        .rev()
        .fold(0, |acc, b| (acc << 1) + b as u32);
    let epsilon = !gamma ^ (u32::MAX << max_length);
    gamma * epsilon
}

#[aoc(day3, part2)]
fn part2((max_length, input): &(u32, Vec<u32>)) -> u32 {
    let (mut oxy_candidates, mut co2_candidates) = (input.clone(), input.clone());
    let (oxy, co2) =
        (1..(max_length + 1))
            .rev()
            .fold((&mut oxy_candidates, &mut co2_candidates), |acc, i| {
                let (should_keep_oxy, should_keep_co2) =
                    (more_bits_set(acc.0, i), !more_bits_set(acc.1, i));
                if acc.0.len() > 1 {
                    acc.0.retain(|x| should_keep_oxy == is_bit_set(*x, i));
                }
                if acc.1.len() > 1 {
                    acc.1.retain(|x| should_keep_co2 == is_bit_set(*x, i));
                }
                (acc.0, acc.1)
            });
    oxy[0] * co2[0]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_bit_set() {
        assert_eq!(is_bit_set(123, 1), true);
        assert_eq!(is_bit_set(127, 3), true);
        assert_eq!(is_bit_set(127, 8), false);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&parse_input(
                "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"
            )),
            198
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse_input(
                "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"
            )),
            230
        );
    }
}
