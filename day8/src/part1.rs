use std::{collections::HashMap, hash::Hash};

use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use parse::QuickRegex;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input).unwrap());
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

pub fn part1(input: &str) -> Result<u64> {
    let (directions, map) = parse(input)?;
    let mut current = "AAA";
    let mut steps = 1;
    for direction in directions.iter().cycle() {
        let (left, right) = map.get(current).pretty()?;
        current = match direction {
            Direction::Left => left,
            Direction::Right => right,
        };
        if current == "ZZZ" {
            break;
        }
        steps += 1;
    }
    Ok(steps)
}

#[cfg(test)]
mod part1_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example1() {
        let input = indoc! {r#"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#};
        assert_eq!(part1(input).expect("part1 should return Ok"), 2);
    }

    #[test]
    fn example2() {
        let input = indoc! {r#"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#};
        assert_eq!(part1(input).expect("part1 should return Ok"), 6);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input).expect("part1 should return Ok"), 0);
    }
}
