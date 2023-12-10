use std::{collections::HashSet, hash::Hash};

use geo::{Contains, Coord, Polygon};
use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use parse::{parse_grid, Grid, Relationship};
use petgraph::{
    algo::{all_simple_paths, dijkstra},
    graphmap::GraphMap,
    visit::Dfs,
    Directed,
};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input).unwrap());
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

pub fn part2(input: &str) -> Result<i64> {
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

    let mut dfs = Dfs::new(&graph, *start);
    let mut pipe_coords = vec![];
    while let Some(next) = dfs.next(&graph) {
        pipe_coords.push(next);
    }

    let pipe = Polygon::new(
        pipe_coords
            .iter()
            .map(|(x, y)| (*x as f64, *y as f64))
            .collect(),
        vec![],
    );

    dbg!(pipe_coords.len());

    let mut contained = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            if pipe_coords.contains(&(x, y)) {
                continue;
            }
            if pipe.contains(&Coord::from((x as f64, y as f64))) {
                contained += 1;
            }
        }
    }

    Ok(contained)
}

#[cfg(test)]
mod part2_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example1() {
        let input = indoc! {r#"
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"#};
        assert_eq!(part2(input).expect("part2 should return Ok"), 4);
    }

    #[test]
    fn example1_2() {
        let input2 = indoc! {r#"
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
"#};
        assert_eq!(part2(input2).expect("part2 should return Ok"), 4);
    }

    #[test]
    fn example2() {
        let input = indoc! {r#"
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"#};
        assert_eq!(part2(input).expect("part2 should return Ok"), 10);
    }

    #[test]
    fn example3() {
        let input = indoc! {r#"
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"#};
        assert_eq!(part2(input).expect("part2 should return Ok"), 8);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input).expect("part2 should return Ok"), 0);
    }
}
