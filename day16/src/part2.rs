use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;

use elsa::FrozenMap;
use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use parse::Grid;
use parse::{QuickRegex, Tile};
use rayon::iter::{ParallelBridge, ParallelIterator};
use uuid::Uuid;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input).unwrap());
}

Tile! {
    Empty = '.',
    HorizontalSplitter = '-',
    VerticalSplitter = '|',
    SlashMirror = '/',
    BackslashMirror = '\\',
}

type G = Grid<Tile>;

fn parse(input: &str) -> Result<G> {
    Tile::parse_grid(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn find_energizes(grid: G, start: ((isize, isize), Direction)) -> usize {
    let mut beams: Vec<((isize, isize), Direction)> = vec![start];
    let mut energized: HashSet<(usize, usize, Direction)> =
        HashSet::with_capacity(grid.width * grid.height);

    while !beams.is_empty() {
        dbg!(beams.len());
        for i in 0..beams.len() {
            let ((x, y), direction) = beams[i];
            let usize_x = x.try_into();
            let usize_y = y.try_into();
            if usize_x.is_err() || usize_y.is_err() {
                beams.remove(i);
                break;
            }
            let usize_x = usize_x.unwrap();
            let usize_y = usize_y.unwrap();
            if grid.validate(usize_x, usize_y).is_err()
                || energized.contains(&(usize_x, usize_y, direction))
            {
                beams.remove(i);
                break;
            }

            energized.insert((usize_x, usize_y, direction));
            let current = grid.get(x as usize, y as usize).unwrap();

            match (current, direction) {
                (Tile::Empty, _)
                | (Tile::HorizontalSplitter, Direction::Right)
                | (Tile::HorizontalSplitter, Direction::Left)
                | (Tile::VerticalSplitter, Direction::Up)
                | (Tile::VerticalSplitter, Direction::Down) => match direction {
                    Direction::Up => beams[i] = ((x, y - 1), direction),
                    Direction::Down => beams[i] = ((x, y + 1), direction),
                    Direction::Left => beams[i] = ((x - 1, y), direction),
                    Direction::Right => beams[i] = ((x + 1, y), direction),
                },

                (Tile::SlashMirror, Direction::Up) => beams[i] = ((x + 1, y), Direction::Right),
                (Tile::SlashMirror, Direction::Down) => beams[i] = ((x - 1, y), Direction::Left),
                (Tile::SlashMirror, Direction::Left) => beams[i] = ((x, y + 1), Direction::Down),
                (Tile::SlashMirror, Direction::Right) => beams[i] = ((x, y - 1), Direction::Up),

                (Tile::BackslashMirror, Direction::Up) => beams[i] = ((x - 1, y), Direction::Left),
                (Tile::BackslashMirror, Direction::Down) => {
                    beams[i] = ((x + 1, y), Direction::Right)
                }
                (Tile::BackslashMirror, Direction::Left) => beams[i] = ((x, y - 1), Direction::Up),
                (Tile::BackslashMirror, Direction::Right) => {
                    beams[i] = ((x, y + 1), Direction::Down)
                }

                (Tile::HorizontalSplitter, Direction::Up) => {
                    beams[i] = ((x - 1, y), Direction::Left);
                    beams.push(((x + 1, y), Direction::Right));
                }

                (Tile::HorizontalSplitter, Direction::Down) => {
                    beams[i] = ((x + 1, y), Direction::Right);

                    beams.push(((x - 1, y), Direction::Left));
                }

                (Tile::VerticalSplitter, Direction::Left) => {
                    beams[i] = ((x, y - 1), Direction::Up);

                    beams.push(((x, y + 1), Direction::Down));
                }

                (Tile::VerticalSplitter, Direction::Right) => {
                    beams[i] = ((x, y + 1), Direction::Down);

                    beams.push(((x, y - 1), Direction::Up));
                }
            }
        }
    }

    let energized = energized.iter().fold(
        HashSet::new(),
        |mut acc: HashSet<(usize, usize)>, (x, y, _)| {
            acc.insert((*x, *y));
            acc
        },
    );

    // dbg!(grid.map(|(coord, _)| if energized.contains(&coord) { '#' } else { '.' }));

    energized.len()
}

pub fn part2(input: &str) -> Result<usize> {
    let grid = parse(input)?;
    (0..grid.width)
        .map(|x| ((x as isize, 0), Direction::Down))
        .chain((0..grid.width).map(|x| ((x as isize, (grid.height - 1) as isize), Direction::Up)))
        .chain((0..grid.height).map(|y| ((0, y as isize), Direction::Right)))
        .chain((0..grid.height).map(|y| (((grid.width - 1) as isize, y as isize), Direction::Left)))
        .par_bridge()
        .map(|start| find_energizes(grid.clone(), start))
        .max()
        .pretty()
}

#[cfg(test)]
mod part2_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#};
        assert_eq!(part2(input).expect("part2 should return Ok"), 51);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input).expect("part2 should return Ok"), 0);
    }
}
