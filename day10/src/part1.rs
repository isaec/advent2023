use std::collections::HashSet;

use miette::Result;
use miette_pretty::Pretty;
use parse::{parse_grid, Grid, QuickRegex, Relationship};
use petgraph::{algo::dijkstra, graphmap::GraphMap, Directed, EdgeType};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input).unwrap());
}

/// | is a vertical pipe connecting north and south.
/// - is a horizontal pipe connecting east and west.
/// L is a 90-degree bend connecting north and east.
/// J is a 90-degree bend connecting north and west.
/// 7 is a 90-degree bend connecting south and west.
/// F is a 90-degree bend connecting south and east.
/// . is ground; there is no pipe in this tile.
/// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl Tile {
    fn get_directions(&self) -> Vec<Direction> {
        match self {
            Tile::Vertical => vec![Direction::N, Direction::S],
            Tile::Horizontal => vec![Direction::E, Direction::W],
            Tile::NorthEast => vec![Direction::N, Direction::E],
            Tile::NorthWest => vec![Direction::N, Direction::W],
            Tile::SouthWest => vec![Direction::S, Direction::W],
            Tile::SouthEast => vec![Direction::S, Direction::E],
            Tile::Ground => vec![],
            Tile::Start => vec![Direction::N, Direction::S, Direction::E, Direction::W],
        }
    }
}

fn parse(input: &str) -> Result<Grid<Tile>> {
    parse_grid(input, |c| match c {
        '|' => Tile::Vertical,
        '-' => Tile::Horizontal,
        'L' => Tile::NorthEast,
        'J' => Tile::NorthWest,
        '7' => Tile::SouthWest,
        'F' => Tile::SouthEast,
        '.' => Tile::Ground,
        'S' => Tile::Start,
        _ => unimplemented!("unknown tile"),
    })
}

pub fn part1(input: &str) -> Result<i64> {
    let grid = parse(input)?;
    let lookup = grid.build_lookup();
    let graph: GraphMap<(usize, usize), i32, Directed> =
        grid.build_graph(&Relationship::Orthogonal, |(a, a_coord), (b, b_coord)| {
            if a == Tile::Ground || b == Tile::Ground {
                return None;
            }

            let a_directions = a.get_directions();
            let b_directions = b.get_directions();

            let (dx, dy) = (
                b_coord.0 as i32 - a_coord.0 as i32,
                b_coord.1 as i32 - a_coord.1 as i32,
            );

            let relationship = match (dx, dy) {
                (0, 0) => unimplemented!("same tile"),
                (0, 1) => {
                    a_directions.contains(&Direction::S) && b_directions.contains(&Direction::N)
                }
                (0, -1) => {
                    a_directions.contains(&Direction::N) && b_directions.contains(&Direction::S)
                }
                (1, 0) => {
                    a_directions.contains(&Direction::E) && b_directions.contains(&Direction::W)
                }
                (-1, 0) => {
                    a_directions.contains(&Direction::W) && b_directions.contains(&Direction::E)
                }
                _ => false,
            };

            if relationship {
                Some(1)
            } else {
                None
            }
        });

    let start = lookup.get(&Tile::Start).pretty()?.first().pretty()?;

    let distances = dijkstra(&graph, *start, None, |_| 1);

    Ok(*distances.values().max().pretty()?)
}

#[cfg(test)]
mod part1_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example1() {
        let input = indoc! {r#"
.....
.S-7.
.|.|.
.L-J.
.....
"#};
        assert_eq!(part1(input).expect("part1 should return Ok"), 4);
    }

    #[test]
    fn example3() {
        let input = indoc! {r#"
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
"#};
        assert_eq!(part1(input).expect("part1 should return Ok"), 4);
    }

    #[test]
    fn example2() {
        let input = indoc! {r#"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"#};
        assert_eq!(part1(input).expect("part1 should return Ok"), 8);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input).expect("part1 should return Ok"), 0);
    }
}
