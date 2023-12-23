use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use parse::{Grid, QuickRegex, Tile};
use petgraph::{algo::all_simple_paths, graphmap::GraphMap, Directed};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input).unwrap());
}

Tile! {
    Path = '.',
    Forest = '#',
    SlopeUp = '^',
    SlopeDown = 'v',
    SlopeLeft = '<',
    SlopeRight = '>',
}

fn parse(input: &str) -> Result<Grid<Tile>> {
    Tile::parse_grid(input)
}

pub fn part1(input: &str) -> Result<u64> {
    let grid = parse(input)?;
    let graph: GraphMap<(usize, usize), (), Directed> = grid.build_graph(
        &parse::Relationship::Orthogonal,
        |(f_tile, f_coord), (t_tile, t_coord)| match (f_tile, t_tile) {
            (Tile::Path, Tile::Path) => Some(()),
            (Tile::Forest, _) => None,
            (_, Tile::Forest) => None,
            (_, Tile::Path) => match (
                f_tile,
                f_coord.0 as isize - t_coord.0 as isize,
                f_coord.1 as isize - t_coord.1 as isize,
            ) {
                (Tile::SlopeUp, 0, 1) => Some(()),
                (Tile::SlopeDown, 0, -1) => Some(()),
                (Tile::SlopeLeft, 1, 0) => Some(()),
                (Tile::SlopeRight, -1, 0) => Some(()),
                _ => None,
            },
            (Tile::Path, _) => Some(()),
            _ => None,
        },
    );

    let start = grid
        .lookup(Tile::Path)
        .iter()
        .min_by_key(|coord| coord.1)
        .unwrap()
        .clone();

    let end = grid
        .lookup(Tile::Path)
        .iter()
        .max_by_key(|coord| coord.1)
        .unwrap()
        .clone();

    dbg!(start, end);

    let paths = all_simple_paths::<Vec<_>, _>(&graph, start, end, 0, None).collect_vec();
    dbg!(paths.len());
    paths.iter().map(|p| p.len() as u64 - 1).max().pretty()
}

#[cfg(test)]
mod part1_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
"#};
        assert_eq!(part1(input).expect("part1 should return Ok"), 94);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input).expect("part1 should return Ok"), 0);
    }
}
