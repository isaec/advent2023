use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use parse::{parse_grid, Grid, QuickRegex, Tile};
use petgraph::{graphmap::GraphMap, Directed};
use strum::{self, IntoEnumIterator};
use strum_macros::EnumIter;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input).unwrap());
}

type G = Grid<u64>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_reverse(self, other: Self) -> bool {
        match (self, other) {
            (Self::Up, Self::Down) => true,
            (Self::Down, Self::Up) => true,
            (Self::Left, Self::Right) => true,
            (Self::Right, Self::Left) => true,
            _ => false,
        }
    }
}

fn parse(input: &str) -> Result<G> {
    parse_grid(input, |c| c.to_digit(10).unwrap() as u64)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node {
    coord: (usize, usize),
    direction: Direction,
    steps_in_direction: u64,
}

pub fn part2(input: &str) -> Result<u64> {
    let grid = parse(input)?;
    let mut graph: GraphMap<Node, u64, Directed> = GraphMap::new();

    let mut frontier = VecDeque::new();
    let origins = [
        Node {
            coord: (0, 0),
            direction: Direction::Down,
            steps_in_direction: 0,
        },
        Node {
            coord: (0, 0),
            direction: Direction::Right,
            steps_in_direction: 0,
        },
    ];
    for &origin in &origins {
        graph.add_node(origin);
        frontier.push_back(origin);
    }

    let goal = (grid.width - 1, grid.height - 1);

    while let Some(node) = frontier.pop_front() {
        Direction::iter()
            .filter(|&d| !d.is_reverse(node.direction))
            .filter_map(|d| {
                let (x, y) = node.coord;
                let coord = match d {
                    Direction::Up if y > 0 => (x, y - 1),
                    Direction::Down => (x, y + 1),
                    Direction::Left if x > 0 => (x - 1, y),
                    Direction::Right => (x + 1, y),
                    _ => return None,
                };

                grid.validate(coord.0, coord.1).ok()?;

                if (d == node.direction && node.steps_in_direction >= 10)
                    || (d != node.direction && node.steps_in_direction < 4)
                {
                    return None;
                }

                if coord == goal && !(d == node.direction && node.steps_in_direction >= 3) {
                    return None;
                }

                Some(Node {
                    coord,
                    direction: d,
                    steps_in_direction: if d == node.direction {
                        node.steps_in_direction + 1
                    } else {
                        1
                    },
                })
            })
            .for_each(|n| {
                if !graph.contains_node(n) {
                    frontier.push_back(n);
                }
                graph.add_node(n);
                let cost = grid.get_tuple(n.coord).unwrap();
                graph.add_edge(node, n, *cost);
            });
    }

    for path in origins
        .iter()
        .map(|&origin| {
            petgraph::algo::astar(&graph, origin, |n| n.coord == goal, |(_, _, e)| *e, |_| 0)
        })
        .map(|p| p.unwrap().1.iter().map(|n| n.coord).collect::<HashSet<_>>())
    {
        Tile! {
            Path = '#',
            Empty = '.',
        }
        dbg!(grid.map(|((x, y), _)| {
            if path.contains(&(x, y)) {
                Tile::Path
            } else {
                Tile::Empty
            }
        }));
    }

    origins
        .iter()
        .map(|&origin| {
            petgraph::algo::astar(&graph, origin, |n| n.coord == goal, |(_, _, e)| *e, |_| 0)
        })
        .map(|p| p.unwrap().0)
        .min()
        .pretty()
}

#[cfg(test)]
mod part2_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
"#};
        assert_eq!(part2(input).expect("part2 should return Ok"), 94);
    }

    #[test]
    fn example_2() {
        let input = indoc! {r#"
111111111111
999999999991
999999999991
999999999991
999999999991
"#};
        assert_eq!(part2(input).expect("part2 should return Ok"), 71);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input).expect("part2 should return Ok"), 0);
    }
}
