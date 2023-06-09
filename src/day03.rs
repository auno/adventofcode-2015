use std::collections::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use anyhow::{Result, anyhow};

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err(anyhow!("Invalid input character: {c}"))
        }
    }
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Result<Vec<Direction>> {
    input
        .chars()
        .map(|c| c.try_into())
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &Vec<Direction>) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    visited.insert((0, 0));
    let mut current_location = (0, 0);

    for &direction in input {
        current_location = move_location(current_location, direction);

        visited.insert(current_location);
    }

    visited.len()
}

fn move_location(current_location: (i32, i32), direction: Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (current_location.0, current_location.1 + 1),
        Direction::Down => (current_location.0, current_location.1 - 1),
        Direction::Left => (current_location.0 - 1, current_location.1),
        Direction::Right => (current_location.0 + 1, current_location.1),
    }
}

#[aoc(day3, part2)]
fn part2(input: &Vec<Direction>) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    visited.insert((0, 0));
    let mut l1 = (0, 0);
    let mut l2 = (0, 0);

    for (d1, d2) in input.iter().tuples() {
        l1 = move_location(l1, *d1);
        l2 = move_location(l2, *d2);

        visited.insert(l1);
        visited.insert(l2);
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(2, part1(&parse(">").unwrap()));
    }

    #[test]
    fn part1_example2() {
        assert_eq!(4, part1(&parse("^>v<").unwrap()));
    }

    #[test]
    fn part1_example3() {
        assert_eq!(2, part1(&parse("^v^v^v^v^v").unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(3, part2(&parse("^v").unwrap()));
    }

    #[test]
    fn part2_example2() {
        assert_eq!(3, part2(&parse("^>v<").unwrap()));
    }

    #[test]
    fn part2_example3() {
        assert_eq!(11, part2(&parse("^v^v^v^v^v").unwrap()));
    }
}