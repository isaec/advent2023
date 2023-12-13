use core::panic;
use std::iter::zip;

use itertools::{equal, Itertools};
use miette::Result;
use miette_pretty::Pretty;
use parse::{parse_grid, Grid, QuickRegex};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input).unwrap());
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

fn find_mirror(grid: &Grid<Tile>) -> Axis {
    let columns = grid.compute_columns();
    let rows = grid.compute_rows();

    for mirror_x in 1..grid.width {
        // test the left and right sides
        if zip(
            columns.iter().take(mirror_x).rev(),
            columns.iter().skip(mirror_x).take(mirror_x),
        )
        .all(|(left, right)| equal(left, right))
        {
            return Axis::X(mirror_x);
        }
    }

    for mirror_y in 1..grid.height {
        // test the top and bottom sides
        if zip(
            rows.iter().take(mirror_y).rev(),
            rows.iter().skip(mirror_y).take(mirror_y),
        )
        .all(|(top, bottom)| equal(top, bottom))
        {
            return Axis::Y(mirror_y);
        }
    }

    dbg!(grid);
    unreachable!("no mirror found")
}

pub fn part1(input: &str) -> Result<usize> {
    let parsed = parse(input)?;
    Ok(parsed
        .iter()
        .map(find_mirror)
        .map(|a| {
            dbg!(match a {
                Axis::Y(y) => dbg!(y) * 100,
                Axis::X(x) => dbg!(x),
            })
        })
        .sum())
}

#[cfg(test)]
mod part1_tests {
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
        assert_eq!(part1(input).expect("part1 should return Ok"), 405);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input).expect("part1 should return Ok"), 0);
    }
}
