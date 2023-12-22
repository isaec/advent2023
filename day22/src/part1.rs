use core::panic;
use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use parse::{Grid, QuickRegex, Tile};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input).unwrap());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(i64, i64, i64);

impl Pos {
    fn new(iter: impl Iterator<Item = i64>) -> Result<Self> {
        let (x, y, z) = iter.collect_tuple().pretty()?;
        Ok(Self(x, y, z))
    }

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s.get_digits()?.into_iter())
    }

    fn iter_to(self, end: Self) -> Box<dyn Iterator<Item = Self>> {
        let Self(x1, y1, z1) = self;
        let Self(x2, y2, z2) = end;
        assert!(
            [x1 == x2, y1 == y2, z1 == z2]
                .into_iter()
                .filter(|&b| b)
                .count()
                == 2
        );
        if x1 != x2 {
            let (x1, x2, rev) = if x1 < x2 {
                (x1, x2, false)
            } else {
                (x2, x1, true)
            };
            let base: Box<dyn Iterator<Item = i64>> = if rev {
                Box::new((x1..=x2).rev())
            } else {
                Box::new(x1..=x2)
            };
            Box::new(base.map(move |x| Self(x, y1, z1)))
        } else if y1 != y2 {
            let (y1, y2, rev) = if y1 < y2 {
                (y1, y2, false)
            } else {
                (y2, y1, true)
            };
            let base: Box<dyn Iterator<Item = i64>> = if rev {
                Box::new((y1..=y2).rev())
            } else {
                Box::new(y1..=y2)
            };
            Box::new(base.map(move |y| Self(x1, y, z1)))
        } else {
            let (z1, z2, rev) = if z1 < z2 {
                (z1, z2, false)
            } else {
                (z2, z1, true)
            };
            let base: Box<dyn Iterator<Item = i64>> = if rev {
                Box::new((z1..=z2).rev())
            } else {
                Box::new(z1..=z2)
            };
            Box::new(base.map(move |z| Self(x1, y1, z)))
        }
    }
}

macro_rules! impl_op {
    ($trait:ident, $fn:ident, $op:tt) => {
        impl std::ops::$trait for Pos {
            type Output = Self;
            fn $fn(self, rhs: Self) -> Self::Output {
                Self(
                    self.0 $op rhs.0,
                    self.1 $op rhs.1,
                    self.2 $op rhs.2,
                )
            }
        }
    };
}

impl_op!(Add, add, +);
impl_op!(Sub, sub, -);

fn parse(input: &str) -> Result<Vec<(usize, Pos, Pos)>> {
    input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let (start, end) = l.split_once('~').pretty()?;
            let start = Pos::from_str(start)?;
            let end = Pos::from_str(end)?;
            Ok((i, start, end))
        })
        .collect()
}

pub fn part1(input: &str) -> Result<i64> {
    let bricks = parse(input)?;
    // let mut grid = HashMap::new();
    // for (i, start, end) in bricks {
    //     let delta = end - start;
    // }
    Ok(0)
}

#[cfg(test)]
mod part1_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
"#};
        assert_eq!(part1(input).expect("part1 should return Ok"), 5);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input).expect("part1 should return Ok"), 0);
    }

    #[test]
    fn iter_to() {
        let start = Pos(0, 0, 0);
        let end = Pos(0, 0, 2);
        let mut iter = start.iter_to(end);
        assert_eq!(iter.next(), Some(Pos(0, 0, 0)));
        assert_eq!(iter.next(), Some(Pos(0, 0, 1)));
        assert_eq!(iter.next(), Some(Pos(0, 0, 2)));
        assert_eq!(iter.next(), None);

        let start = Pos(10, 0, 0);
        let end = Pos(-3, 0, 0);
        let mut iter = start.iter_to(end);
        assert_eq!(
            iter.collect_vec(),
            vec![
                Pos(10, 0, 0),
                Pos(9, 0, 0),
                Pos(8, 0, 0),
                Pos(7, 0, 0),
                Pos(6, 0, 0),
                Pos(5, 0, 0),
                Pos(4, 0, 0),
                Pos(3, 0, 0),
                Pos(2, 0, 0),
                Pos(1, 0, 0),
                Pos(0, 0, 0),
                Pos(-1, 0, 0),
                Pos(-2, 0, 0),
                Pos(-3, 0, 0)
            ]
        );
    }
}
