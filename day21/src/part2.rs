use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use parse::{pattern_enum, Grid, Tile};
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

    dbg!(steps % grid.width as u64);

    dbg!(start);

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

    let mut currently_at_map: HashMap<(isize, isize), HashSet<(usize, usize)>> = HashMap::new();
    currently_at_map.insert((0, 0), [start].iter().copied().collect());

    for step in 0..steps {
        // dbg!(step);
        let mut next_map = HashMap::new();
        for (tile_coord, currently_at) in currently_at_map.iter() {
            for current_coord in currently_at.iter() {
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

                    // let pair = (neighbor, next_tile_coord);

                    // if !next.contains(&pair) {
                    //     next.insert(pair);
                    // }

                    next_map
                        .entry(next_tile_coord)
                        .or_insert_with(|| HashSet::new())
                        .insert(neighbor);
                }
            }
        }
        currently_at_map = next_map;
        pattern_enum! {
            enum At {
                At = "O",
                Garden = ".",
                Rock = "#",
            }
        }
        if step == 65 {
            grid.map(|(coord, tile)| {
                if currently_at_map.get(&(0, 0)).unwrap().contains(&coord) {
                    At::At
                } else {
                    match tile {
                        Tile::GardenPlot => At::Garden,
                        Tile::Rock => At::Rock,
                        Tile::Start => At::Garden,
                    }
                }
            })
            .debug_to_file("f1")?;
            dbg!(step);
            dbg!(steps % step);
            break;
        }
    }

    Ok(currently_at_map
        .iter()
        .map(|(_, currently_at)| currently_at.len())
        .sum::<usize>() as u64)
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
            // (6, 16),
            (10, 50),
            // (50, 1594),
            // (100, 6536),
            // (500, 167004),
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
