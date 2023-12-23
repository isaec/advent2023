use std::{
    collections::{HashMap, HashSet},
    fmt::Formatter,
};

use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use parse::QuickRegex;
use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input).unwrap());
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
        if self == end {
            return Box::new(std::iter::once(self));
        }
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

impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pos({}, {}, {})", self.0, self.1, self.2)
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

fn fall<T>(grid: &mut HashSet<Pos>, initial_pos: &Vec<Pos>, with_new: &mut T) -> bool
where
    T: FnMut(&Pos),
{
    let mut pos = initial_pos.clone();

    for p in pos.iter_mut() {
        grid.remove(p);
    }

    let is_vert = pos
        .iter()
        .take(2)
        .tuple_windows()
        .all(|(a, b)| a.0 == b.0 && a.1 == b.1);

    // dbg!(is_vert, pos.len());

    if is_vert {
        let mut min = *pos.iter().min_by_key(|p| p.2).unwrap();
        // dbg!(min);
        while min.2 > 1 && grid.get(&(min + Pos(0, 0, -1))).is_none() {
            for p in pos.iter_mut() {
                *p = *p + Pos(0, 0, -1);
            }
            min = min + Pos(0, 0, -1);
        }
    } else {
        while pos[0].2 > 1
            && pos
                .iter()
                .all(|p| grid.get(&(*p + Pos(0, 0, -1))).is_none())
        {
            for p in pos.iter_mut() {
                *p = *p + Pos(0, 0, -1);
            }
        }
    }

    for p in pos.iter() {
        grid.insert(*p);
        with_new(p);
    }

    initial_pos != &pos
}

fn fall_all<'a, T>(
    grid: &mut HashSet<Pos>,
    initial_pos: &HashMap<usize, Vec<Pos>>,
    lowest_first_ids: impl Iterator<Item = &'a usize>,
    with_new: &mut T,
) -> u64
where
    T: FnMut(&Pos, &usize),
{
    lowest_first_ids
        .map(|i| fall(grid, &initial_pos[i], &mut |p| with_new(p, i)) as u64)
        .sum()
}

fn compute_lowest_first_ids(initial_pos: &HashMap<usize, Vec<Pos>>) -> Vec<usize> {
    initial_pos
        .iter()
        .sorted_unstable_by_key(|(_, pos_vec)| pos_vec.iter().min_by_key(|p| p.2).unwrap().2)
        .map(|(i, _)| i)
        .copied()
        .collect()
}

pub fn part2(input: &str) -> Result<u64> {
    let bricks = parse(input)?;
    let mut grid = HashSet::new();
    let mut initial_pos = HashMap::new();

    for (i, start, end) in &bricks {
        for pos in start.iter_to(*end) {
            grid.insert(pos);
        }
        initial_pos.insert(*i, start.iter_to(*end).collect_vec());
    }

    let mut post_fall_pos = HashMap::new();

    fall_all(
        &mut grid,
        &initial_pos,
        compute_lowest_first_ids(&initial_pos).iter(),
        &mut |p: &Pos, i: &usize| {
            post_fall_pos.entry(*i).or_insert_with(Vec::new).push(*p);
        },
    );

    let lowest_first_ids = compute_lowest_first_ids(&initial_pos);

    // figure out how many bricks can be removed without fall_all changing the grid
    Ok(bricks
        .iter()
        .map(|(i, _, _)| *i)
        .par_bridge()
        .map(|i| {
            let mut grid_copy: HashSet<Pos> = grid.clone();
            for pos in &post_fall_pos[&i] {
                grid_copy.remove(pos);
            }
            fall_all(
                &mut grid_copy,
                &post_fall_pos,
                lowest_first_ids.iter().filter(|&&i2| i2 != i),
                &mut |_, _| {},
            )
        })
        .sum())
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input).expect("part2 should return Ok"), 63166);
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
        let iter = start.iter_to(end);
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
