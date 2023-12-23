use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use parse::{Grid, QuickRegex, Tile};
use petgraph::{algo::all_simple_paths, graphmap::GraphMap, Directed};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input).unwrap());
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

pub fn part2(input: &str) -> Result<u64> {
    let grid = parse(input)?;
    let graph: GraphMap<(usize, usize), (), Directed> = grid.build_graph(
        &parse::Relationship::Orthogonal,
        |(f_tile, f_coord), (t_tile, t_coord)| match (f_tile, t_tile) {
            (Tile::Path, Tile::Path) => Some(()),
            (Tile::Forest, _) => None,
            (_, Tile::Forest) => None,
            (_, Tile::Path) => Some(()),
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

    let mut max_len = 0;
    all_simple_paths::<Vec<_>, _>(&graph, start, end, 6221, None).for_each(|path| {
        let len = path.len() as u64 - 1;
        if len > max_len {
            max_len = len;
            dbg!(max_len);
        }
    });

    Ok(max_len)
}

#[cfg(test)]
mod part2_tests {
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
        assert_eq!(part2(input).expect("part2 should return Ok"), 154);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input).expect("part2 should return Ok"), 0);
    }
}
