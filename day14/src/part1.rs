use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use parse::{QuickRegex, Tile};

use parse::Grid;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input).unwrap());
}

Tile! {
    Round = 'O',
    Cube = '#',
    Empty = '.',
}

fn parse(input: &str) -> Result<Grid<Tile>> {
    Tile::parse_grid(input)
}

fn roll_rocks(grid: &mut Grid<Tile>) -> Result<()> {
    for coord in grid
        .lookup(Tile::Round)
        .iter()
        .sorted_by(|a, b| a.1.cmp(&b.1))
    {
        grid.slide_while(*coord, (0, -1), |_, t| *t == Tile::Empty, Tile::Empty)?
    }

    Ok(())
}

fn compute_load(grid: &Grid<Tile>) -> i64 {
    let rows = grid.compute_rows();

    rows.iter()
        .enumerate()
        .map(|(y, row)| {
            let round_rocks = row.iter().filter(|t| ***t == Tile::Round).count() as i64;
            let distance_from_south = (rows.len() - y) as i64;
            round_rocks * distance_from_south
        })
        .sum::<i64>()
}

pub fn part1(input: &str) -> Result<i64> {
    let mut grid = parse(input)?;
    dbg!(&grid);
    roll_rocks(&mut grid);
    dbg!(&grid);
    Ok(compute_load(&grid))
}

#[cfg(test)]
mod part1_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"#};
        assert_eq!(part1(input).expect("part1 should return Ok"), 136);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input).expect("part1 should return Ok"), 0);
    }
}
