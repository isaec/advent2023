use std::collections::HashSet;

use miette::Result;
use miette_pretty::Pretty;
use parse::{pattern_enum, Grid, QuickRegex, Tile};
use petgraph::{graphmap::GraphMap, Directed, Undirected};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input, 64).unwrap());
}

/// Start counts as a GardenPlot
Tile! {
    Start = 'S',
    GardenPlot = '.',
    Rock = '#',
}

type G = Grid<Tile>;

fn parse(input: &str) -> Result<G> {
    Tile::parse_grid(input)
}

pub fn part1(input: &str, steps: u64) -> Result<u64> {
    let grid = parse(input)?;
    let starts = grid.lookup(Tile::Start);
    assert!(starts.len() == 1);
    let start = starts[0];
    let mut graph = GraphMap::<(usize, usize), (), Undirected>::new();

    let mut frontier = vec![start];

    while let Some(current) = frontier.pop() {
        let neighbors = grid.get_neighbors(current.0, current.1)?;
        for neighbor in neighbors.iter(&parse::Relationship::Orthogonal) {
            let tile = *grid.get(neighbor.0, neighbor.1)?;

            if tile == Tile::Rock {
                continue;
            }

            if tile == Tile::GardenPlot && !graph.contains_node(neighbor) {
                frontier.push(neighbor);
                graph.add_edge(current, neighbor, ());
            }
        }
    }

    pattern_enum! {
        enum Status {
            At = "O",
            NotAt = ".",
            Rock = "#",
        }
    }
    let mut currently_at = HashSet::new();
    currently_at.insert(start);

    for _ in 0..steps {
        let mut next = HashSet::new();
        for current in currently_at.iter() {
            let neighbors = grid.get_neighbors(current.0, current.1)?;
            for neighbor in neighbors.iter(&parse::Relationship::Orthogonal) {
                let tile = *grid.get(neighbor.0, neighbor.1)?;

                if tile == Tile::Rock {
                    continue;
                }

                if !next.contains(&neighbor) {
                    next.insert(neighbor);
                }
            }
        }
        currently_at = next;
    }

    Ok(currently_at.len() as u64)
}

#[cfg(test)]
mod part1_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
"#};
        assert_eq!(part1(input, 6).expect("part1 should return Ok"), 16);
    }
}
