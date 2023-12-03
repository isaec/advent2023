use std::collections::HashSet;

use itertools::Itertools;
use miette::Result;

// add pretty and pretty_msg to Result to show the line number where the error occurred
use miette_pretty::Pretty;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input).unwrap());
}

#[derive(Debug)]
struct JoinedNumber {
    x_range: (usize, usize),
    y: usize,
    n: u64,
}

fn parse(input: &str) -> Result<(Vec<JoinedNumber>, Vec<(usize, usize)>)> {
    // get the numbers adjacent to a non . symbol
    let lines = input.lines().collect::<Vec<_>>();

    let symbol_regex = fancy_regex::Regex::new(r"[^.\d]").pretty_msg("regex")?;

    let symbol_cords = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| symbol_regex.is_match(&c.to_string()).unwrap())
                .map(move |(x, _)| (x, y))
        })
        .collect::<Vec<_>>();

    let numbers = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| c.is_digit(10))
                .map(move |(x, c)| (x, y, c.to_digit(10).unwrap()))
        })
        .collect::<Vec<_>>();

    // join all the numbers that are next to each other
    let mut joined_numbers = vec![];

    let mut next = 0;
    for i in 0..numbers.len() {
        if i < next {
            continue;
        }
        let (x, y, n) = numbers[i];
        let mut joined = vec![(x, y, n)];
        let mut ended = false;
        for j in (i + 1)..numbers.len() {
            let (x2, y2, n2) = numbers[j];
            if (x2 == x + (j - i)) && (y2 == y) {
                joined.push((x2, y2, n2));
            } else {
                next = j;
                ended = true;
                break;
            }
        }
        if !ended {
            next = numbers.len();
        }

        joined_numbers.push(JoinedNumber {
            x_range: (joined[0].0, joined[joined.len() - 1].0),
            y,
            n: joined
                .iter()
                .map(|(_, _, n)| *n)
                .join("")
                .parse::<u64>()
                .pretty()?,
        });
    }

    dbg!(&joined_numbers);

    Ok((joined_numbers, symbol_cords))
}

pub fn part1(input: &str) -> Result<u64> {
    let (numbers, symbols) = parse(input)?;
    let adjacent_numbers = numbers
        .iter()
        .filter(|n| {
            let (x1, x2) = n.x_range;
            symbols
                .iter()
                .any(|(x, y)| // check if the symbol is adjacent to the number
                    // vertical adjacent
                    ((x1 <= *x && *x <= x2) && (*y == n.y || *y == n.y + 1 || *y == usize::checked_sub(n.y, 1).unwrap_or(n.y))) ||
                    // horizontal adjacent
                    ((n.y == *y) && (*x == x1 || *x == x2 || *x == usize::checked_sub(x1, 1).unwrap_or(x1) || *x == x2 + 1)) ||
                    // diagonal adjacent
                    ((n.y == *y + 1 || n.y == usize::checked_sub(*y, 1).unwrap_or(*y)) && (*x == x1 + 1 || *x == usize::checked_sub(x1, 1).unwrap_or(x1) || *x == x2 + 1 || *x == usize::checked_sub(x2, 1).unwrap_or(x2)))
                    
                )
        })
        .collect::<Vec<_>>();

    dbg!(&adjacent_numbers);

    Ok(adjacent_numbers
        .iter()
        .map(|n| n.n)
        .sum::<u64>())
}

#[cfg(test)]
mod part1_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#};
        dbg!(&input);
        assert_eq!(part1(input).expect("part1 should return Ok"), 4361);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input).expect("part1 should return Ok"), 0);
    }
}
