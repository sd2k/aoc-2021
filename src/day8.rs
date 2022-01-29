// TODO: memoize a bunch of this to avoid loads of recalculations and allocations.
//
// Segment counts:
//
// - top left: 6
// - top: 8
// - top right: 8
// - middle: 7
// - bottom left: 4
// - bottom: 7
// - bottom right: 9
//
// - 0: 6
// - 1: 2
// - 2: 5
// - 3: 5
// - 4: 4
// - 5: 5
// - 6: 6
// - 7: 3
// - 8: 7
// - 9: 6
use std::{collections::HashMap, ops::Deref};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

trait IterExt: Iterator + Sized {
    fn find_only<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        let mut filtered = self.filter(predicate);
        let res = filtered.next();
        debug_assert!(filtered.next().is_none());
        res
    }
}

impl<T> IterExt for T where T: Iterator + Sized {}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Pattern(Vec<char>);

impl Pattern {
    fn new(mut chars: Vec<char>) -> Self {
        // Sort for easier comparison later.
        // TODO: might not be needed?
        chars.sort_unstable();
        Self(chars)
    }

    fn is_one(&self) -> bool {
        self.len() == 2
    }

    fn is_four(&self) -> bool {
        self.len() == 4
    }

    fn is_seven(&self) -> bool {
        self.len() == 3
    }

    fn is_eight(&self) -> bool {
        self.len() == 7
    }
}

impl Deref for Pattern {
    type Target = [char];

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

#[derive(Clone, Debug)]
struct Entry {
    patterns: [Pattern; 10],
    output: [Pattern; 4],
    letter_counts: HashMap<char, usize>,
}

impl Entry {
    fn new(patterns: [Pattern; 10], output: [Pattern; 4]) -> Self {
        let letter_counts = patterns
            .iter()
            .flat_map(|pattern| pattern.iter().copied())
            .counts();
        Self {
            patterns,
            output,
            letter_counts,
        }
    }

    fn simple(&self, n: usize) -> &Pattern {
        self.patterns
            .iter()
            .find_only(|p| p.len() == n)
            .expect("each simple number should appear exactly once")
    }

    fn letters_with_count(&self, n: usize) -> impl Iterator<Item = char> + '_ {
        self.letter_counts
            .iter()
            .filter_map(move |(k, v)| (*v == n).then(|| *k))
    }

    fn letter_with_count(&self, n: usize) -> char {
        self.letters_with_count(n).next().unwrap()
    }

    fn top(&self) -> char {
        let one = self.one();
        *self
            .seven()
            .iter()
            .find_only(|c| !one.contains(c))
            .expect("there should be one char difference between seven and one")
    }

    fn top_left(&self) -> char {
        self.letter_with_count(6)
    }

    fn top_right(&self) -> char {
        self.letters_with_count(8)
            .find_only(|x| *x != self.top())
            .unwrap()
    }

    fn middle(&self) -> char {
        let (one, four) = (self.one(), self.four());
        let top_left = self.top_left();
        *four
            .iter()
            .find_only(|c| !one.contains(c) && **c != top_left)
            .unwrap()
    }

    fn bottom_left(&self) -> char {
        self.letter_with_count(4)
    }

    #[allow(dead_code)]
    fn bottom(&self) -> char {
        self.letters_with_count(7)
            .find_only(|x| *x != self.middle())
            .unwrap()
    }

    fn bottom_right(&self) -> char {
        self.letter_with_count(9)
    }

    fn one(&self) -> &Pattern {
        self.simple(2)
    }

    fn two(&self) -> &Pattern {
        let bottom_left = self.bottom_left();
        self.patterns
            .iter()
            .find_only(|x| x.len() == 5 && x.contains(&bottom_left))
            .unwrap()
    }

    fn three(&self) -> &Pattern {
        let top_right = self.top_right();
        let bottom_right = self.bottom_right();
        self.patterns
            .iter()
            .find_only(|x| x.len() == 5 && x.contains(&bottom_right) && x.contains(&top_right))
            .unwrap()
    }

    fn four(&self) -> &Pattern {
        self.simple(4)
    }

    fn five(&self) -> &Pattern {
        let top_left = self.top_left();
        self.patterns
            .iter()
            .find_only(|x| x.len() == 5 && x.contains(&top_left))
            .unwrap()
    }

    fn six(&self) -> &Pattern {
        let top_right = self.top_right();
        self.patterns
            .iter()
            .find_only(|x| x.len() == 6 && !x.contains(&top_right))
            .unwrap()
    }

    fn seven(&self) -> &Pattern {
        self.simple(3)
    }

    fn eight(&self) -> &Pattern {
        self.simple(7)
    }

    fn nine(&self) -> &Pattern {
        let (middle, top_right) = (self.middle(), self.top_right());
        self.patterns
            .iter()
            .find_only(|x| x.len() == 6 && x.contains(&top_right) && x.contains(&middle))
            .unwrap()
    }

    fn zero(&self) -> &Pattern {
        let middle = self.middle();
        self.patterns
            .iter()
            .find_only(|p| p.len() == 6 && !p.contains(&middle))
            .expect("zero should appear once")
    }

    fn pattern_lookup(&self) -> HashMap<&Pattern, usize> {
        let mut m = HashMap::with_capacity(10);
        m.insert(self.one(), 1);
        m.insert(self.two(), 2);
        m.insert(self.three(), 3);
        m.insert(self.four(), 4);
        m.insert(self.five(), 5);
        m.insert(self.six(), 6);
        m.insert(self.seven(), 7);
        m.insert(self.eight(), 8);
        m.insert(self.nine(), 9);
        m.insert(self.zero(), 0);
        m
    }

    fn output(&self) -> usize {
        let lookup = self.pattern_lookup();
        self.output
            .iter()
            .rev()
            .enumerate()
            .map(|(i, pattern)| *lookup.get(&pattern).unwrap() * 10usize.pow(i as u32))
            .sum()
    }
}

type Input = Vec<Entry>;

fn get_patterns<const N: usize>(s: &str) -> [Pattern; N] {
    let mut iter = s
        .split(' ')
        .map(|s| Pattern::new(s.trim().chars().collect()));
    [(); N].map(|_| iter.next().expect("enough patterns in section"))
}

fn parse_line(line: &str) -> Entry {
    let (patterns, rest) = line.split_once(" | ").expect("invalid entry");
    Entry::new(get_patterns(patterns), get_patterns(rest))
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Input {
    input.lines().map(parse_line).collect()
}

#[aoc(day8, part1)]
fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|entry| {
            entry
                .output
                .iter()
                .filter(|pattern| {
                    pattern.is_one()
                        || pattern.is_four()
                        || pattern.is_seven()
                        || pattern.is_eight()
                })
                .count()
        })
        .sum()
}

#[aoc(day8, part2)]
fn part2(input: &Input) -> usize {
    input.iter().map(|entry| entry.output()).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&parse_input(
                "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
            )),
            26
        );
    }

    #[test]
    fn test_part2_small() {
        let entries = parse_input(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        let e = &entries[0];
        assert_eq!(e.top_left(), 'e');
        assert_eq!(e.top(), 'd');
        assert_eq!(e.top_right(), 'a');
        assert_eq!(e.middle(), 'f');
        assert_eq!(e.bottom_left(), 'g');
        assert_eq!(e.bottom(), 'c');
        assert_eq!(e.bottom_right(), 'b');
        assert_eq!(part2(&entries), 5353);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse_input(
                "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
            )),
            61229
        );
    }
}
