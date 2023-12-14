use std::collections::HashMap;

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

fn roll_rocks_north(grid: &mut Grid<Tile>) -> Result<()> {
    let lookup = grid.build_lookup();
    let rolling = lookup.get(&Tile::Round).pretty()?;

    let rolling = rolling
        .iter()
        .sorted_by(|a, b| {
            let a = a.1;
            let b = b.1;
            a.cmp(&b)
        })
        .copied()
        .collect_vec();

    for (x, y) in rolling {
        grid.set(x, y, Tile::Empty);
        let mut replaced = false;
        for test_y in (0..y).rev() {
            let test = grid.get(x, test_y)?;
            if *test != Tile::Empty {
                grid.set(x, test_y + 1, Tile::Round);
                replaced = true;
                break;
            }
        }
        if !replaced {
            grid.set(x, 0, Tile::Round);
        }
    }

    Ok(())
}

fn roll_rocks_west(grid: &mut Grid<Tile>) -> Result<()> {
    let lookup = grid.build_lookup();
    let rolling = lookup.get(&Tile::Round).pretty()?;

    let rolling = rolling
        .iter()
        .sorted_by(|a, b| {
            let a = a.0;
            let b = b.0;
            a.cmp(&b)
        })
        .copied()
        .collect_vec();

    for (x, y) in rolling {
        grid.set(x, y, Tile::Empty);
        let mut replaced = false;
        for test_x in (0..x).rev() {
            let test = grid.get(test_x, y)?;
            if *test != Tile::Empty {
                grid.set(test_x + 1, y, Tile::Round);
                replaced = true;
                break;
            }
        }
        if !replaced {
            grid.set(0, y, Tile::Round);
        }
    }

    Ok(())
}

fn roll_rocks_south(grid: &mut Grid<Tile>) -> Result<()> {
    let lookup = grid.build_lookup();
    let rolling = lookup.get(&Tile::Round).pretty()?;

    let rolling = rolling
        .iter()
        .sorted_by(|a, b| {
            let a = a.1;
            let b = b.1;
            b.cmp(&a)
        })
        .copied()
        .collect_vec();

    for (x, y) in rolling {
        grid.set(x, y, Tile::Empty);
        let mut replaced = false;
        for test_y in y + 1..grid.height {
            let test = grid.get(x, test_y)?;
            if *test != Tile::Empty {
                grid.set(x, test_y - 1, Tile::Round);
                replaced = true;
                break;
            }
        }
        if !replaced {
            grid.set(x, grid.height - 1, Tile::Round);
        }
    }

    Ok(())
}

fn roll_rocks_east(grid: &mut Grid<Tile>) -> Result<()> {
    let lookup = grid.build_lookup();
    let rolling = lookup.get(&Tile::Round).pretty()?;

    let rolling = rolling
        .iter()
        .sorted_by(|a, b| {
            let a = a.0;
            let b = b.0;
            b.cmp(&a)
        })
        .copied()
        .collect_vec();

    for (x, y) in rolling {
        grid.set(x, y, Tile::Empty);
        let mut replaced = false;
        for test_x in x + 1..grid.width {
            let test = grid.get(test_x, y)?;
            if *test != Tile::Empty {
                grid.set(test_x - 1, y, Tile::Round);
                replaced = true;
                break;
            }
        }
        if !replaced {
            grid.set(grid.width - 1, y, Tile::Round);
        }
    }

    Ok(())
}

fn spin_cycle(grid: &mut Grid<Tile>) -> Result<()> {
    roll_rocks_north(grid)?;
    roll_rocks_west(grid)?;
    roll_rocks_south(grid)?;
    roll_rocks_east(grid)?;
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
    let mut grids: HashMap<_, usize> = HashMap::new();

    let mut i: usize = 1;
    loop {
        let load = compute_load(&grid);
        spin_cycle(&mut grid)?;
        if let Some(prev_iter) = grids.insert(grid.clone(), i) {
            let cycle_length = i - prev_iter;
            let remaining = 1000000000 - i;
            let remaining = remaining % cycle_length;
            for _ in 0..remaining {
                spin_cycle(&mut grid)?;
            }
            break Ok(compute_load(&grid));
        }
        i += 1;
    }
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
        assert_eq!(part1(input).expect("part1 should return Ok"), 64);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input).expect("part1 should return Ok"), 0);
    }
}
