use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use parse::parse_grid;
use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input, 1_000_000).unwrap());
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
enum Tile {
    E,
    G,
}

fn parse(input: &str, expansion: usize) -> Result<Vec<(usize, usize)>> {
    let grid = parse_grid(input, |c| match c {
        '.' => Tile::E,
        '#' => Tile::G,
        _ => panic!("invalid tile: {}", c),
    })?;
    let lookup = grid.build_lookup();
    let coords = lookup.get(&Tile::G).pretty()?;
    Ok(expand_gaps(&coords, expansion))
}

fn expand_gaps(coords: &[(usize, usize)], expansion: usize) -> Vec<(usize, usize)> {
    let mut new_coords = vec![];

    let mut last_x = 0;
    let x_sorted_coords = coords.iter().sorted_by_key(|(x, _)| *x);
    for (x, y) in x_sorted_coords {
        let last_x_expanded = new_coords.last().map(|(x, _)| *x).unwrap_or(0);
        new_coords.push((
            last_x_expanded
                + (x - last_x)
                + ((x - last_x).checked_sub(1).unwrap_or(0) * expansion)
                    .checked_sub(1)
                    .unwrap_or(0),
            *y,
        ));
        last_x = *x;
    }

    let mut final_coords = vec![];

    let mut last_y = 0;
    let y_sorted_coords = new_coords.iter().sorted_by_key(|(_, y)| *y);
    for (x, y) in y_sorted_coords {
        let last_y_expanded = final_coords.last().map(|(_, y)| *y).unwrap_or(0);
        final_coords.push((
            *x,
            last_y_expanded
                + (y - last_y)
                + ((y - last_y).checked_sub(1).unwrap_or(0) * expansion)
                    .checked_sub(1)
                    .unwrap_or(0),
        ));
        last_y = *y;
    }

    final_coords
}

fn manhattan_distance(a: &(usize, usize), b: &(usize, usize)) -> i64 {
    (a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs()
}

pub fn part2(input: &str, expansion: usize) -> Result<i64> {
    let hashes = parse(input, expansion)?;
    Ok(hashes
        .iter()
        .combinations(2)
        .par_bridge()
        .map(|v| {
            let (a, b) = (v[0], v[1]);
            manhattan_distance(a, b)
        })
        .sum())
}

#[cfg(test)]
mod part2_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example_1mill() {
        let input = indoc! {r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#};
        assert_eq!(
            part2(input, 1_000_000).expect("part2 should return Ok"),
            82000210
        );
    }

    #[test]
    fn example_10() {
        let input = indoc! {r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#};
        assert_eq!(part2(input, 10).expect("part2 should return Ok"), 1030);
    }

    #[test]
    fn oli_example_1mill() {
        let input = include_str!("../oli_input.txt");
        let actual = part2(input, 1_000_000).expect("part2 should return Ok");
        let expected = 634324905172;
        dbg!(actual - expected);
        assert_eq!(actual, expected,);
    }

    #[test]
    fn seb_example_1mill() {
        let input = include_str!("../seb_input.txt");
        let actual = part2(input, 1_000_000).expect("part2 should return Ok");
        let expected = 827009909817;
        dbg!(actual - expected);
        assert_eq!(actual, expected,);
    }
}
