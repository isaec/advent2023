use geo::{Area, Contains, Coord, GeodesicArea, Polygon};
use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use parse::{Grid, QuickRegex, Tile};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input).unwrap());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse(input: &str) -> Result<Vec<(Direction, i64)>> {
    input
        .lines()
        .map(|l| {
            let (_, dist_color) = l.split_at(1);
            let (_, color) = dist_color.trim().split_once(' ').unwrap();
            let color = color.trim_start_matches("(#").trim_end_matches(')');
            // #70c710 = R 461937
            let dist: i64 = i64::from_str_radix(&color[0..5], 16).unwrap();
            let direction = match dbg!(color.chars().last().unwrap()) {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => unreachable!(),
            };
            Ok(dbg!((direction, dist)))
        })
        .collect()
}

pub fn part2(input: &str) -> Result<u64> {
    let parsed = parse(input)?;

    let mut position = (0, 0);
    let mut polygon_points = vec![position];
    let mut boundary_points = 0;
    for (dir, dist) in parsed {
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
mod part2_tests {
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
        assert_eq!(part2(input).expect("part2 should return Ok"), 952408144115);
    }
}
