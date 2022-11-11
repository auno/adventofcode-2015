use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day02)]
fn parse(input: &str) -> Vec<(u32, u32, u32)> {
    input.lines()
        .map(|line| line.trim())
        .flat_map(|line| line.split("x"))
        .map(|num| num.parse::<u32>().unwrap())
        .tuples()
        .collect()
}

#[aoc(day02, part1)]
fn part1(input: &Vec<(u32, u32, u32)>) -> u32 {
    input.into_iter()
        .map(|(l, w, h)| {
            let sides = [ l*w, w*h, h*l ];
            let min = sides.into_iter().min().unwrap_or_default();

            sides.into_iter().sum::<u32>() * 2 + min
        })
        .sum()
}

#[aoc(day02, part2)]
fn part2(input: &Vec<(u32, u32, u32)>) -> u32 {
    input.into_iter()
        .map(|(l, w, h)| {
            let sides = [ 2*(l+w), 2*(w+h), 2*(h+l) ];
            let min = sides.into_iter().min().unwrap_or_default();

            min + l*w*h
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(58, part1(&parse("2x3x4")));
    }

    #[test]
    fn part1_example2() {
        assert_eq!(43, part1(&parse("1x1x10")));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(34, part2(&parse("2x3x4")));
    }

    #[test]
    fn part2_example2() {
        assert_eq!(14, part2(&parse("1x1x10")));
    }
}