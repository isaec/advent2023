use core::panic;
use std::iter::zip;

use itertools::{equal, Itertools};
use miette::Result;
use miette_pretty::Pretty;
use parse::{parse_grid, Grid, QuickRegex};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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

    (1..grid.width)
        .filter(|mirror_x| !matches!(ignore, Some(Axis::X(ignore_x)) if *mirror_x == ignore_x))
        .find(|mirror_x| {
            zip(
                columns.iter().take(*mirror_x).rev(),
                columns.iter().skip(*mirror_x).take(*mirror_x),
            )
            .all(|(left, right)| equal(left, right))
        })
        .map_or_else(
            || {
                (1..grid.height)
                .filter(
                    |mirror_y| !matches!(ignore, Some(Axis::Y(ignore_y)) if *mirror_y == ignore_y),
                )
                .find(|mirror_y| {
                    zip(
                        rows.iter().take(*mirror_y).rev(),
                        rows.iter().skip(*mirror_y).take(*mirror_y),
                    )
                    .all(|(top, bottom)| equal(top, bottom))
                })
                .map(|mirror_y| Axis::Y(mirror_y))
            },
            |mirror_x| Some(Axis::X(mirror_x)),
        )
}

pub fn part2(input: &str) -> Result<usize> {
    let parsed = parse(input)?;
    Ok(parsed
        .par_iter()
        .map(|g| {
            let initial_mirror = find_mirror(g, None).expect("no initial mirror found");
            g.iter()
                .map(|((x, y), _)| {
                    g.clone_replace_at(x, y, |t| match t {
                        Tile::Ash => Tile::Rock,
                        Tile::Rock => Tile::Ash,
                    })
                })
                .find_map(|grid| find_mirror(&grid, Some(initial_mirror)))
                .expect("no solution found")
        })
        .map(|a| match a {
            Axis::Y(y) => y * 100,
            Axis::X(x) => x,
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
        assert_eq!(part2(input).expect("part2 should return Ok"), 31954);
    }
}
