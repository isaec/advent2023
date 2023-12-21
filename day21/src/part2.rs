use std::collections::HashSet;

use itertools::Itertools;
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

    let mut currently_at = HashSet::with_capacity(1_000);
    currently_at.insert((start, (0, 0)));

    for step in 0..steps {
        dbg!(step);
        let mut next = HashSet::with_capacity(currently_at.len());
        for (current_coord, tile_coord) in currently_at.iter() {
            let neighbors = grid
                .get_neighbors_wrapping(current_coord.0, current_coord.1)
                .unwrap();
            for neighbor in neighbors.iter(&parse::Relationship::Orthogonal) {
                let tile = *grid.get(neighbor.0, neighbor.1)?;

                if tile == Tile::Rock {
                    continue;
                }

                let mut next_tile_coord = *tile_coord;

                if (neighbor.0 as isize - current_coord.0 as isize).abs() > 1 {
                    if neighbor.0 == 0 {
                        next_tile_coord.0 -= 1;
                    } else if neighbor.0 == grid.width - 1 {
                        next_tile_coord.0 += 1;
                    }
                }

                if (neighbor.1 as isize - current_coord.1 as isize).abs() > 1 {
                    if neighbor.1 == 0 {
                        next_tile_coord.1 -= 1;
                    } else if neighbor.1 == grid.height - 1 {
                        next_tile_coord.1 += 1;
                    }
                }

                let pair = (neighbor, next_tile_coord);

                if !next.contains(&pair) {
                    next.insert(pair);
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
            // (1000, 668697),
            // (5000, 16733044),
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
