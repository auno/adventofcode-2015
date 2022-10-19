use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> String {
    input.to_string()
}

#[aoc(day1, part1)]
fn part1(s: &str) -> isize {
    s.chars()
        .fold(0, |floor, c| match c {
            '(' => floor + 1,
            ')' => floor - 1,
            _ => panic!("Invalid input: {}", c)
        })
}

#[aoc(day1, part2)]
fn part2(s: &str) -> isize {
    s.chars()
        .fold((0, 0), |(floor, position), c| {
            match (c, floor) {
                (_, -1) => (floor, position),
                ('(', _) => (floor + 1, position + 1),
                (')', _) => (floor - 1, position + 1),
                _ => panic!("Invalid input: {}", c)
            }
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(0, part1("(())"));
    }

    #[test]
    fn part1_example2() {
        assert_eq!(0, part1("()()"));
    }

    #[test]
    fn part1_example3() {
        assert_eq!(3, part1("((("));
    }

    #[test]
    fn part1_example4() {
        assert_eq!(3, part1("(()(()("));
    }

    #[test]
    fn part1_example5() {
        assert_eq!(3, part1("))((((("));
    }

    #[test]
    fn part1_example6() {
        assert_eq!(-1, part1("())"));
    }

    #[test]
    fn part1_example7() {
        assert_eq!(-1, part1("))("));
    }

    #[test]
    fn part1_example8() {
        assert_eq!(-3, part1(")))"));
    }

    #[test]
    fn part1_example9() {
        assert_eq!(-3, part1(")())())"));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(1, part2(")"));
    }

    #[test]
    fn part2_example2() {
        assert_eq!(5, part2("()())"));
    }
}