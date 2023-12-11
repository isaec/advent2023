use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use parse::{parse_grid, Grid, QuickRegex};
use petgraph::{algo::astar, Undirected};
use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input).unwrap());
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
enum Tile {
    E,
    G,
}

fn expand_empty_rows_columns(input: &str) -> String {
    let mut rows = vec![];
    for line in input.lines() {
        rows.push(line);
        if line.chars().all(|c| c == '.') {
            rows.push(line);
        }
    }

    let new_input = rows.join("\n");

    // expand the columns that are empty
    let mut columns = vec![];
    for x in 0..new_input.lines().next().unwrap().len() {
        let mut column = vec![];
        for line in new_input.lines() {
            column.push(line.chars().nth(x).unwrap());
        }
        if (&column).iter().all(|c| *c == '.') {
            columns.push(column.clone());
        }
        columns.push(column.clone());
    }

    let mut new_input = vec![];
    for x in 0..columns[0].len() {
        let mut line: Vec<String> = vec![];
        for column in &columns {
            line.push(column[x].into());
        }
        new_input.push(line.join(""));
    }

    new_input.join("\n")
}

fn parse(input: &str) -> Result<Grid<Tile>> {
    let expanded = expand_empty_rows_columns(input);
    parse_grid(expanded.as_str(), |c| match c {
        '.' => Tile::E,
        '#' => Tile::G,
        _ => panic!("invalid tile: {}", c),
    })
}

fn manhattan_distance(a: &(usize, usize), b: &(usize, usize)) -> i64 {
    ((a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs())
}

pub fn part1(input: &str) -> Result<i64> {
    let initial_grid = parse(input)?;
    let graph: petgraph::prelude::GraphMap<(usize, usize), i32, Undirected> =
        initial_grid.build_graph(&parse::Relationship::Orthogonal, |_, _| Some(1));
    let lookup = initial_grid.build_lookup();
    let hashes = lookup.get(&Tile::G).pretty()?;
    Ok(hashes
        .iter()
        .combinations(2)
        .par_bridge()
        .map(|v| {
            let (a, b) = (v[0], v[1]);
            manhattan_distance(a, b)
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
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#};
        assert_eq!(part1(input).expect("part1 should return Ok"), 374);
    }

    #[test]
    fn expand() {
        let input = indoc! {r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#};

        let expected = indoc! {r#"
....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#......."#};
        assert_eq!(expand_empty_rows_columns(input), expected);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input).expect("part1 should return Ok"), 0);
    }
}
