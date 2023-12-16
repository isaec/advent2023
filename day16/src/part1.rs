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
use uuid::Uuid;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input).unwrap());
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

pub fn part1(input: &str) -> Result<usize> {
    let grid = parse(input)?;
    // dbg!(&grid);
    let mut beams: Vec<((isize, isize), Direction)> = vec![((0, 0), Direction::Right)];
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
            let current = grid.get(x as usize, y as usize)?;

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

    Ok(energized.len())
}

#[cfg(test)]
mod part1_tests {
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
        assert_eq!(part1(input).expect("part1 should return Ok"), 46);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input).expect("part1 should return Ok"), 0);
    }
}
