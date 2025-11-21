use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use md5::{Digest, Md5};
use rayon::prelude::*;

type Input = String;

#[aoc_generator(day4)]
fn parse(input: &str) -> Result<Input> {
    Ok(input.trim().to_string())
}

fn solve(input: &Input, six_digits: bool) -> Option<usize> {
    (0..usize::MAX)
        .into_par_iter()
        .by_exponential_blocks()
        .find_first(|i| {
            let hash = Md5::digest(format!("{input}{i}"));
            hash[0] == 0 && hash[1] == 0 && ((six_digits && hash[2] == 0) || (!six_digits && hash[2] <= 0x0f))
        })
}

#[aoc(day4, part1)]
fn part1(input: &Input) -> Option<usize> {
    solve(input, false)
}

#[aoc(day4, part2)]
fn part2(input: &Input) -> Option<usize> {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "abcdef";
    const EXAMPLE2: &str = "pqrstuv";

    #[test]
    fn part1_example1() {
        assert_eq!(Some(609043), part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_example2() {
        assert_eq!(Some(1048970), part1(&parse(EXAMPLE2).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(Some(117946), part1(&parse(include_str!("../input/2015/day4.txt")).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(Some(3938038), part2(&parse(include_str!("../input/2015/day4.txt")).unwrap()));
    }
}
