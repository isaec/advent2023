use core::panic;
use std::iter::zip;

use itertools::{equal, Itertools};
use miette::Result;
use miette_pretty::Pretty;
use parse::{parse_grid, Grid, QuickRegex};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input).unwrap());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Ash,
    Rock,
}

fn parse(input: &str) -> Result<Vec<Grid<Tile>>> {
    input
        .split("\n\n")
        .map(|g| {
            parse_grid(g, |c| match c {
                '#' => Tile::Rock,
                '.' => Tile::Ash,
                _ => panic!("unexpected char, {c}"),
            })
        })
        .try_collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Axis {
    X(usize),
    Y(usize),
}

fn display_tiles(tiles: &[&Tile]) -> String {
    tiles
        .iter()
        .map(|t| match t {
            Tile::Ash => '.',
            Tile::Rock => '#',
        })
        .collect()
}

fn find_mirror(grid: &Grid<Tile>, ignore: Option<Axis>) -> Option<Axis> {
    let columns = grid.compute_columns();
    let rows = grid.compute_rows();

    for mirror_x in 1..grid.width {
        if let Some(Axis::X(ignore_x)) = ignore {
            if mirror_x == ignore_x {
                continue;
            }
        }
        // test the left and right sides
        if zip(
            columns.iter().take(mirror_x).rev(),
            columns.iter().skip(mirror_x).take(mirror_x),
        )
        .all(|(left, right)| equal(left, right))
        {
            return Some(Axis::X(mirror_x));
        }
    }

    for mirror_y in 1..grid.height {
        if let Some(Axis::Y(ignore_y)) = ignore {
            if mirror_y == ignore_y {
                continue;
            }
        }
        // test the top and bottom sides
        if zip(
            rows.iter().take(mirror_y).rev(),
            rows.iter().skip(mirror_y).take(mirror_y),
        )
        .all(|(top, bottom)| equal(top, bottom))
        {
            return Some(Axis::Y(mirror_y));
        }
    }

    None
}

pub fn part2(input: &str) -> Result<usize> {
    let parsed = parse(input)?;
    Ok(parsed
        .iter()
        .map(|g| {
            let initial_mirror = find_mirror(g, None).expect("no initial mirror found");
            for x in 0..g.width {
                for y in 0..g.height {
                    let mut grid: Grid<Tile> = g.clone();

                    let current = grid.get(x, y).unwrap();
                    let next = match current {
                        Tile::Ash => Tile::Rock,
                        Tile::Rock => Tile::Ash,
                    };
                    let index = grid.index(x, y);
                    grid.data[index] = next;

                    if let Some(mirror) = find_mirror(&grid, Some(initial_mirror)) {
                        if mirror != initial_mirror {
                            return mirror;
                        }
                    }
                }
            }
            dbg!(g);
            unreachable!("no solution found")
        })
        .map(|a| {
            dbg!(match a {
                Axis::Y(y) => dbg!(y) * 100,
                Axis::X(x) => dbg!(x),
            })
        })
        .sum())
}

#[cfg(test)]
mod part2_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"#};
        assert_eq!(part2(input).expect("part2 should return Ok"), 400);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input).expect("part2 should return Ok"), 0);
    }
}
