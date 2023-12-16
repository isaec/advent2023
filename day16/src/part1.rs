use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

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

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn part1(input: &str) -> Result<usize> {
    let grid = parse(input)?;
    dbg!(&grid);
    let mut beams: Vec<((isize, isize), Direction, Uuid)> =
        vec![((0, 0), Direction::Right, Uuid::new_v4())];
    let mut energized: FrozenMap<Uuid, Box<RefCell<HashSet<(usize, usize)>>>> = FrozenMap::new();
    energized.insert(beams[0].2, Box::new(RefCell::new(HashSet::new())));

    while !beams.is_empty() {
        for i in 0..beams.len() {
            let ((x, y), direction, uuid) = beams[i];
            let usize_x = x.try_into();
            let usize_y = y.try_into();
            if usize_x.is_err() || usize_y.is_err() {
                beams.remove(i);
                break;
            }
            let usize_x = usize_x.unwrap();
            let usize_y = usize_y.unwrap();
            let uuid_energized = energized.get(&uuid).pretty()?;
            if grid.validate(usize_x, usize_y).is_err()
                || uuid_energized.borrow().contains(&(usize_x, usize_y))
            {
                beams.remove(i);
                break;
            }

            uuid_energized.borrow_mut().insert((usize_x, usize_y));
            let current = grid.get(x as usize, y as usize)?;

            let new_energized = uuid_energized.clone();
            let insert_new = |uuid| {
                energized.borrow_mut().insert(uuid, Box::new(new_energized));
            };

            match (current, direction) {
                (Tile::Empty, _)
                | (Tile::HorizontalSplitter, Direction::Right)
                | (Tile::HorizontalSplitter, Direction::Left)
                | (Tile::VerticalSplitter, Direction::Up)
                | (Tile::VerticalSplitter, Direction::Down) => match direction {
                    Direction::Up => beams[i] = ((x, y - 1), direction, uuid),
                    Direction::Down => beams[i] = ((x, y + 1), direction, uuid),
                    Direction::Left => beams[i] = ((x - 1, y), direction, uuid),
                    Direction::Right => beams[i] = ((x + 1, y), direction, uuid),
                },

                (Tile::SlashMirror, Direction::Up) => {
                    beams[i] = ((x + 1, y), Direction::Right, uuid)
                }
                (Tile::SlashMirror, Direction::Down) => {
                    beams[i] = ((x - 1, y), Direction::Left, uuid)
                }
                (Tile::SlashMirror, Direction::Left) => {
                    beams[i] = ((x, y + 1), Direction::Down, uuid)
                }
                (Tile::SlashMirror, Direction::Right) => {
                    beams[i] = ((x, y - 1), Direction::Up, uuid)
                }

                (Tile::BackslashMirror, Direction::Up) => {
                    beams[i] = ((x - 1, y), Direction::Left, uuid)
                }
                (Tile::BackslashMirror, Direction::Down) => {
                    beams[i] = ((x + 1, y), Direction::Right, uuid)
                }
                (Tile::BackslashMirror, Direction::Left) => {
                    beams[i] = ((x, y - 1), Direction::Up, uuid)
                }
                (Tile::BackslashMirror, Direction::Right) => {
                    beams[i] = ((x, y + 1), Direction::Down, uuid)
                }

                (Tile::HorizontalSplitter, Direction::Up) => {
                    beams[i] = ((x - 1, y), Direction::Left, uuid);
                    let new_uuid = Uuid::new_v4();
                    beams.push(((x + 1, y), Direction::Right, new_uuid));
                    insert_new(new_uuid);
                }

                (Tile::HorizontalSplitter, Direction::Down) => {
                    beams[i] = ((x + 1, y), Direction::Right, uuid);
                    let new_uuid = Uuid::new_v4();
                    beams.push(((x - 1, y), Direction::Left, new_uuid));
                    insert_new(new_uuid);
                }

                (Tile::VerticalSplitter, Direction::Left) => {
                    beams[i] = ((x, y - 1), Direction::Up, uuid);
                    let new_uuid = Uuid::new_v4();
                    beams.push(((x, y + 1), Direction::Down, new_uuid));
                    insert_new(new_uuid);
                }

                (Tile::VerticalSplitter, Direction::Right) => {
                    beams[i] = ((x, y + 1), Direction::Down, uuid);
                    let new_uuid = Uuid::new_v4();
                    beams.push(((x, y - 1), Direction::Up, new_uuid));
                    insert_new(new_uuid);
                }
            }
        }
    }

    let energized = energized.into_map().iter().fold(
        HashSet::new(),
        |mut acc: HashSet<(usize, usize)>, (_, v)| {
            acc.extend(v.borrow().iter());
            acc
        },
    );

    grid.visualize(|_, coord| if energized.contains(&coord) { '#' } else { '.' });

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
