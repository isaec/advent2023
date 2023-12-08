use std::{collections::HashMap, hash::Hash};

use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use num::integer::lcm;
use parse::QuickRegex;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input).unwrap());
}

fn parse(input: &str) -> Result<(Vec<Direction>, HashMap<&str, (&str, &str)>)> {
    let (directions, map) = input.split_once("\n\n").pretty()?;

    let directions = dbg!(directions)
        .chars()
        .map(|s| match s {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!("Invalid direction {}", s),
        })
        .collect();

    let map: Vec<_> = map
        .lines()
        .map(|line| {
            let (key, value) = line.split_once(" = ").pretty()?;
            let lr = value.get_matches("\\w+")?;
            Ok((key, (lr[0], lr[1])))
        })
        .collect::<Result<_>>()?;

    let mut hashmap: HashMap<&str, (&str, &str)> = HashMap::new();
    for (key, value) in map.into_iter() {
        hashmap.insert(key, (value.0, value.1));
    }

    Ok((directions, hashmap))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
}

fn step<'a>(
    current: &str,
    direction: &Direction,
    map: &HashMap<&str, (&'a str, &'a str)>,
) -> Result<&'a str> {
    let (left, right) = map.get(current).pretty()?;
    Ok(match direction {
        Direction::Left => left,
        Direction::Right => right,
    })
}

pub fn part2(input: &str) -> Result<u64> {
    let (directions, map) = parse(input)?;
    dbg!((&directions, &map));
    let nodes_ending_in_a = map.keys().filter(|key| key.ends_with('A')).collect_vec();
    let cycle_time = nodes_ending_in_a
        .par_iter()
        .map(|node| {
            let mut current = **node;
            let mut steps: u64 = 1;
            for direction in directions.iter().cycle() {
                current = step(current, direction, &map)?;
                if current.ends_with('Z') {
                    break;
                }
                steps += 1;
            }
            Ok(dbg!(steps))
        })
        .collect::<Result<Vec<_>>>()?;

    // determine the smallest number that is a multiple of all cycle times (lcm)
    cycle_time
        .iter()
        .map(|&x| x)
        .reduce(|a, b| lcm(a, b))
        .pretty()
}

#[cfg(test)]
mod part2_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#};
        assert_eq!(part2(input).expect("part2 should return Ok"), 6);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input).expect("part2 should return Ok"), 0);
    }
}
