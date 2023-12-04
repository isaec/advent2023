use std::collections::HashSet;

use miette::Result;

// add pretty and pretty_msg to Result to show the line number where the error occurred
use miette_pretty::Pretty;
use parse::QuickRegex;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input).unwrap());
}

fn parse(input: &str) -> Result<Vec<(HashSet<i64>, HashSet<i64>)>> {
    input
        .lines()
        .map(|l| {
            let (numbers, winners) = l.split_once("|").pretty()?;
            let numbers = numbers.get_match(r":.+")?.get_digits()?;
            let winners = winners.get_digits()?;
            Ok((HashSet::from_iter(numbers), HashSet::from_iter(winners)))
        })
        .collect()
}

pub fn part1(input: &str) -> Result<u64> {
    let parsed = parse(input)?;
    Ok(parsed
        .iter()
        .map(|(w, m)| {
            let intersections = w.intersection(m).count() as u64;
            if intersections == 0 {
                0
            } else {
                2_u64.pow((intersections - 1) as u32)
            }
        })
        .sum::<u64>())
}

#[cfg(test)]
mod part1_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#};
        assert_eq!(part1(input).expect("part1 should return Ok"), 13);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input).expect("part1 should return Ok"), 21213);
    }
}
