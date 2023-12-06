use std::iter::zip;

use miette::Result;
use miette_pretty::Pretty;
use parse::QuickRegex;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input).unwrap());
}

fn parse(input: &str) -> Result<Vec<(i64, i64)>> {
    let digits: Vec<Vec<i64>> = input
        .lines()
        .map(|l| l.get_digits())
        .collect::<Result<_>>()?;

    Ok(zip(digits[0].clone(), digits[1].clone()).collect())
}

fn find_choices(race: (i64, i64)) -> Vec<i64> {
    let (time, record) = race;
    (1..time)
        .filter_map(|hold_time| {
            let distance = hold_time * (time - hold_time);
            dbg!(hold_time);
            dbg!(distance);
            if distance > record {
                Some(hold_time)
            } else {
                None
            }
        })
        .collect()
}

pub fn part1(input: &str) -> Result<i64> {
    let races = parse(input)?;
    Ok(races
        .iter()
        .map(|r| find_choices(*r).len() as i64)
        .product())
}

#[cfg(test)]
mod part1_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
Time:      7  15   30
Distance:  9  40  200
"#};
        assert_eq!(part1(input).expect("part1 should return Ok"), 288);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input).expect("part1 should return Ok"), 0);
    }
}
