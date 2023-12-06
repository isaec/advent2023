use std::iter::zip;

use miette::Result;
use miette_pretty::Pretty;
use parse::QuickRegex;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input).unwrap());
}

fn parse(input: &str) -> Result<(i64, i64)> {
    let digits: Vec<i64> = input
        .lines()
        .map(|l| l.get_digits()?.first().pretty().map(|d| *d))
        .collect::<Result<_>>()?;

    Ok((digits[0], digits[1]))
}

fn find_choices(race: (i64, i64)) -> usize {
    let (time, record) = race;
    (1..time)
        .into_par_iter()
        .filter_map(|hold_time| {
            let distance = hold_time * (time - hold_time);
            // dbg!(hold_time);
            // dbg!(distance);
            if distance > record {
                Some(hold_time)
            } else {
                None
            }
        })
        .count()
}

pub fn part2(input: &str) -> Result<usize> {
    let races = parse(input)?;
    Ok(find_choices(races))
}

#[cfg(test)]
mod part2_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
Time:      71530
Distance:  940200
"#};
        assert_eq!(part2(input).expect("part2 should return Ok"), 71503);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input).expect("part2 should return Ok"), 0);
    }
}
