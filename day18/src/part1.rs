use geo::{Area, Contains, Coord, GeodesicArea, Polygon};
use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use parse::{Grid, QuickRegex, Tile};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input).unwrap());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse(input: &str) -> Result<Vec<(Direction, f64, String)>> {
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
            let dist = dist
                .parse::<f64>()
                .pretty_msg(format!("dist: {:?}", dist))?;
            let color = color.trim_start_matches("(#").trim_end_matches(')');
            Ok((dir, dist, color.to_string()))
        })
        .collect()
}

pub fn part1(input: &str) -> Result<u64> {
    let parsed = parse(input)?;

    let mut position = (0f64, 0f64);
    let mut polygon_points = vec![position];
    let mut boundary_points = 0;
    for (dir, dist, color) in parsed {
        match dir {
            Direction::Up => {
                position.1 -= dist;
            }
            Direction::Down => {
                position.1 += dist;
            }
            Direction::Left => {
                position.0 -= dist;
            }
            Direction::Right => {
                position.0 += dist;
            }
        }
        boundary_points += dist as u64;
        polygon_points.push(position);
    }

    let polygon = Polygon::new(
        polygon_points
            .iter()
            .map(|(x, y)| (*x as f64, *y as f64))
            .collect(),
        vec![],
    );

    // use picks theorem to calculate area

    let area = polygon.unsigned_area();
    dbg!(boundary_points);
    dbg!(area);
    let interior_points = area + (boundary_points as f64 / 2.0) + 1.0;

    Ok(interior_points as u64)
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
        assert_eq!(part1(input).expect("part1 should return Ok"), 62);
    }
}
