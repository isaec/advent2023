use std::{cell::RefCell, collections::HashMap, hash::Hash};

use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use parse::QuickRegex;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input).unwrap());
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

fn parse(input: &str) -> Result<Vec<(Vec<State>, Vec<i64>)>> {
    input
        .lines()
        .map(|l| {
            let (condition_record, contiguous_damaged_size) = l.split_once(" ").pretty()?;
            let condition_record = vec![condition_record].repeat(5).join("?");
            let condition_record = condition_record
                .chars()
                .map(|c| match c {
                    '?' => State::Unknown,
                    '.' => State::Operational,
                    '#' => State::Damaged,
                    _ => unreachable!(),
                })
                .collect();
            let contiguous_damaged_size = contiguous_damaged_size.get_digits()?.repeat(5);
            Ok((condition_record, contiguous_damaged_size))
        })
        .try_collect()
}

#[derive(Eq, Hash, PartialEq)]
struct CacheKey {
    damaged_signature: Vec<usize>,
    last_state: State,
}

fn cached_compute_possible_arrangements<'a>(
    condition_record: Vec<State>,
    contiguous_damaged_size: &[i64],
    cache: &RefCell<HashMap<CacheKey, i64>>,
    cache_key: CacheKey,
) -> i64 {
    if let Some(value) = cache.borrow().get(&cache_key) {
        // dbg!(value);
        return *value;
    }

    let value = compute_possible_arrangements(condition_record, contiguous_damaged_size, cache);

    if value == 0 {
        return 0;
    }

    cache.borrow_mut().insert(cache_key, value);

    value
}

fn compute_possible_arrangements(
    condition_record: Vec<State>,
    contiguous_damaged_size: &[i64],
    cache: &RefCell<HashMap<CacheKey, i64>>,
) -> i64 {
    let grouped = condition_record.iter().group_by(|s| **s);
    // dbg!(states);

    let mut damaged_signature = vec![];
    let contains_unknown = condition_record.contains(&State::Unknown);
    for (state, group) in grouped.into_iter() {
        if state == State::Unknown {
            break;
        }
        if state == State::Damaged {
            let expected_size = contiguous_damaged_size.get(damaged_signature.len());
            if let Some(expected_size) = expected_size {
                let size = group.count();
                if contains_unknown {
                    if size > *expected_size as usize {
                        return 0;
                    }
                } else {
                    if size != *expected_size as usize {
                        return 0;
                    }
                }
                damaged_signature.push(size);
            } else {
                return 0;
            }
        }
    }
    if !contains_unknown && damaged_signature.len() != contiguous_damaged_size.len() {
        return 0;
    }

    let mut i = 0;
    while i < condition_record.len() {
        let condition = condition_record[i];
        match condition {
            State::Operational | State::Damaged => {
                i += 1;
            }
            State::Unknown => {
                let mut operational_condition_record = condition_record.clone();
                operational_condition_record[i] = State::Operational;
                let mut damaged_condition_record = condition_record;
                damaged_condition_record[i] = State::Damaged;

                return cached_compute_possible_arrangements(
                    operational_condition_record,
                    contiguous_damaged_size,
                    cache,
                    CacheKey {
                        damaged_signature: damaged_signature.clone(),
                        last_state: State::Operational,
                    },
                ) + cached_compute_possible_arrangements(
                    damaged_condition_record,
                    contiguous_damaged_size,
                    cache,
                    CacheKey {
                        damaged_signature: damaged_signature.clone(),
                        last_state: State::Damaged,
                    },
                );
            }
        }
    }

    1
}

pub fn part2(input: &str) -> Result<i64> {
    let parsed = parse(input)?;
    Ok(parsed
        .par_iter()
        .map(|(c, d)| {
            dbg!(compute_possible_arrangements(
                c.clone(),
                d,
                &RefCell::new(HashMap::new())
            ))
        })
        .sum())
}

#[cfg(test)]
mod part2_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"#};
        assert_eq!(part2(input).expect("part2 should return Ok"), 525152);
    }

    #[test]
    fn example_line3() {
        let input = indoc! {r#"
?#?#?#?#?#?#?#? 1,3,1,6
"#};
        assert_eq!(part2(input).expect("part2 should return Ok"), 1);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input).expect("part2 should return Ok"), 0);
    }
}
