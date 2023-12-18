use geo::{Contains, Coord, Polygon};
use miette::Result;
use miette_pretty::Pretty;
use parse::{Grid, QuickRegex, Tile};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input, 1000).unwrap());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse(input: &str) -> Result<Vec<(Direction, usize, String)>> {
    input
        .lines()
        .map(|l| {
            let (dir, dist_color) = l.split_at(1);
            let dir = match dir {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!(),
            };
            let (dist, color) = dist_color.trim().split_once(' ').pretty()?;
            let dist = dist.parse().pretty_msg(format!("dist: {:?}", dist))?;
            let color = color.trim_start_matches("(#").trim_end_matches(')');
            Ok((dir, dist, color.to_string()))
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum GridTile {
    Empty,
    Edge(String),
    Interior,
}

pub fn part1(input: &str, grid_size: usize) -> Result<usize> {
    let parsed = parse(input)?;
    let mut grid = Grid {
        width: grid_size,
        height: grid_size,
        data: vec![GridTile::Empty; grid_size * grid_size + 1],
    };

    let mut position = (grid_size / 2, grid_size / 2);
    let mut polygon_points = vec![position];
    for (dir, dist, color) in parsed {
        let tile = GridTile::Edge(color);
        dbg!(position, dir, dist);
        match dir {
            Direction::Up => {
                for y in position.1 - dist..=position.1 {
                    grid.set(position.0, y, tile.clone());
                }
                position.1 -= dist;
            }
            Direction::Down => {
                for y in position.1..=position.1 + dist {
                    grid.set(position.0, y, tile.clone());
                }
                position.1 += dist;
            }
            Direction::Left => {
                for x in position.0 - dist..position.0 {
                    grid.set(x, position.1, tile.clone());
                }
                position.0 -= dist;
            }
            Direction::Right => {
                for x in position.0..position.0 + dist {
                    grid.set(x, position.1, tile.clone());
                }
                position.0 += dist;
            }
        }
        polygon_points.push(position);
    }

    // {
    //     Tile! {
    //         Empty = '.',
    //         Fill = '#',
    //     }
    //     dbg!(grid.map(|(_, t)| match t {
    //         GridTile::Empty => Tile::Empty,
    //         GridTile::Edge(_) => Tile::Fill,
    //         GridTile::Interior => Tile::Fill,
    //     }));
    // }

    let polygon = Polygon::new(
        polygon_points
            .iter()
            .map(|(x, y)| (*x as f64, *y as f64))
            .collect(),
        vec![],
    );

    for (x, y) in grid.lookup(GridTile::Empty) {
        if polygon.contains(&Coord::from((x as f64, y as f64))) {
            grid.set(x, y, GridTile::Interior);
        }
    }

    Ok(grid.lookup(GridTile::Interior).len()
        + grid.lookup_filter(|t| matches!(t, GridTile::Edge(_))).len())
}

#[cfg(test)]
mod part1_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"#};
        assert_eq!(part1(input, 20).expect("part1 should return Ok"), 62);
    }
}
