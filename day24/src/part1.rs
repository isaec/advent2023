use std::ops::RangeInclusive;

use f128::f128;
use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use num::Float;
use parse::QuickRegex;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input, 200000000000000..=400000000000000).unwrap());
}

type Triple = (i64, i64, i64);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Line {
    position: Triple,
    velocity: Triple,
}

fn parse(input: &str) -> Result<Vec<Line>> {
    input
        .lines()
        .map(|l| {
            let (position, velocity) = l.split_once('@').pretty()?;
            let position = position
                .get_digits()?
                .into_iter()
                .collect_tuple()
                .pretty()?;
            let velocity = velocity
                .get_digits()?
                .into_iter()
                .collect_tuple()
                .pretty()?;
            Ok(Line { position, velocity })
        })
        .collect()
}

fn find_intersection_2d(
    (x1, y1): (f128, f128),
    (x2, y2): (f128, f128),
    (x3, y3): (f128, f128),
    (x4, y4): (f128, f128),
) -> Option<(f128, f128)> {
    // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line
    let x_num = (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4);
    let x_den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    let y_num = (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4);
    let y_den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    if x_den == 0.0.into() || y_den == 0.0.into() {
        return None;
    }

    let x = x_num / x_den;
    let y = y_num / y_den;

    Some((x, y))
}

impl Line {
    fn get_xy_intersection(&self, other: &Line) -> Option<(f128, f128)> {
        let intersection = find_intersection_2d(
            (self.position.0.into(), self.position.1.into()),
            (
                (self.position.0 + self.velocity.0).into(),
                (self.position.1 + self.velocity.1).into(),
            ),
            (other.position.0.into(), other.position.1.into()),
            (
                (other.position.0 + other.velocity.0).into(),
                (other.position.1 + other.velocity.1).into(),
            ),
        )?;
        // determine if the intersection is in the past (only future intersections count)
        if (intersection.0 - f128::new(self.position.0)).signum()
            != f128::new(self.velocity.0).signum()
            || (intersection.1 - f128::new(self.position.1)).signum()
                != f128::new(self.velocity.1).signum()
            || (intersection.0 - f128::new(other.position.0)).signum()
                != f128::new(other.velocity.0).signum()
            || (intersection.1 - f128::new(other.position.1)).signum()
                != f128::new(other.velocity.1).signum()
        {
            return None;
        }

        Some(intersection)
    }
}

pub fn part1(input: &str, bound: RangeInclusive<i64>) -> Result<usize> {
    let parsed = parse(input)?;
    let bound = f128::new(*bound.start())..=f128::new(*bound.end());
    Ok(parsed
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b)| a.get_xy_intersection(b))
        .filter(|(x, y)| bound.contains(x) && bound.contains(y))
        .count())
}

#[cfg(test)]
mod part1_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
"#};
        assert_eq!(part1(input, 7..=27).expect("part1 should return Ok"), 2);
    }

    #[test]
    fn seb_example() {
        let input = indoc! {r#"
273139584437103, 287334362311499, 281777562457473 @ 26, 28, -30
321192696696425, 339042958161682, 224857547314094 @ -65, -63, 8
"#};
        assert_eq!(
            part1(input, 200000000000000..=400000000000000).expect("part1 should return Ok"),
            1
        );
    }
}
