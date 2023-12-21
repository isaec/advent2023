use std::collections::HashSet;

use miette::Result;
use miette_pretty::Pretty;
use parse::{Grid, Tile};
use petgraph::{graphmap::GraphMap, Undirected};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input, 26501365).unwrap());
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

pub fn part2(input: &str, steps: u64) -> Result<u64> {
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

    let mut currently_at = HashSet::new();
    currently_at.insert(start);

    for _ in 0..steps {
        let mut next = HashSet::new();
        for current in currently_at.iter() {
            let neighbors = grid.get_neighbors_wrapping(current.0, current.1)?;
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
mod part2_tests {
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
        [
            (6, 16),
            (10, 50),
            (50, 1594),
            (100, 6536),
            (500, 167004),
            (1000, 668697),
            (5000, 16733044),
        ]
        .iter()
        .for_each(|(steps, expected)| {
            assert_eq!(
                part2(input, *steps).expect("part2 should return Ok"),
                *expected,
                "for {steps} steps expected {expected}"
            );
        });
    }
}
