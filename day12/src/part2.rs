use std::{hash::Hash, iter::zip};

use elsa::FrozenMap;
use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use parse::QuickRegex;

fn main() {
    let input = include_str!("../input.txt");
    // TODO: CHANGE THIS TO 5
    dbg!(part2(input, 1).unwrap());
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

fn parse(input: &str, repeats: usize) -> Result<Vec<(Vec<State>, Vec<u64>)>> {
    input
        .lines()
        .map(|l| {
            let (condition_record, contiguous_damaged_size) = l.split_once(" ").pretty()?;
            let condition_record = if repeats != 0 {
                vec![condition_record].repeat(repeats).join("?")
            } else {
                condition_record.to_string()
            };
            let condition_record = condition_record
                .chars()
                .map(|c| match c {
                    '?' => State::Unknown,
                    '.' => State::Operational,
                    '#' => State::Damaged,
                    _ => unreachable!(),
                })
                .collect();
            let contiguous_damaged_size = contiguous_damaged_size
                .get_digits()?
                .iter()
                .map(|v| *v as u64)
                .collect_vec();
            Ok((
                condition_record,
                if repeats != 0 {
                    contiguous_damaged_size.repeat(repeats)
                } else {
                    contiguous_damaged_size
                },
            ))
        })
        .try_collect()
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct CacheKey {
    damage_signature: Vec<Damage>,
}

fn cached_compute_possible_arrangements(
    condition_record: Vec<State>,
    contiguous_damaged_size: &[u64],
    cache: &FrozenMap<CacheKey, Box<i64>>,
    cache_key: CacheKey,
) -> i64 {
    if let Some(value) = cache.get(&cache_key) {
        // dbg!(value);
        return *value;
    }

    let value = compute_possible_arrangements(condition_record, contiguous_damaged_size, cache);

    // cache.insert(cache_key, Box::new(value));

    value
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Damage {
    Exact(u64),
    Range(u64, u64),
    Optional,
}

impl Damage {
    fn bump_upper_bound(self, by: u64) -> Self {
        match self {
            Damage::Exact(size) => Damage::Range(size, size + by),
            Damage::Range(lower, upper) => Damage::Range(lower, upper + by),
            Damage::Optional => todo!(),
        }
    }
}

fn contains(damage: Damage, size: u64) -> bool {
    match damage {
        Damage::Exact(damage_size) => damage_size == size,
        Damage::Range(lower, upper) => lower <= size && size <= upper,
        Damage::Optional => true,
    }
}

fn render_states(states: &[State]) -> String {
    states
        .iter()
        .map(|s| match s {
            State::Operational => ".",
            State::Damaged => "#",
            State::Unknown => "?",
        })
        .collect()
}

fn build_damage_signature(condition_record: &Vec<State>) -> Vec<Damage> {
    let grouped = condition_record.iter().group_by(|s| **s);
    // dbg!(states);

    let mut damaged_signature: Vec<Damage> = vec![];
    let mut last = None;
    for (state, group) in grouped.into_iter() {
        let size = group.count() as u64;
        match state {
            State::Unknown => match last {
                Some(State::Unknown) | Some(State::Damaged) => {
                    let last = damaged_signature.pop().unwrap();
                    damaged_signature.push(last.bump_upper_bound(size));
                }
                Some(State::Operational) => {
                    damaged_signature.push(Damage::Optional);
                    break;
                }
                _ => break,
            },
            State::Damaged => {
                if matches!(damaged_signature.last(), Some(Damage::Range(_, _))) {
                    let last = damaged_signature.pop().unwrap();
                    damaged_signature.push(last.bump_upper_bound(size));
                } else {
                    damaged_signature.push(Damage::Exact(size));
                }
            }
            State::Operational => {
                if matches!(damaged_signature.last(), Some(Damage::Range(_, _))) {
                    break;
                }
            }
        }
        last = Some(state);
    }

    damaged_signature
}

fn compute_possible_arrangements(
    condition_record: Vec<State>,
    contiguous_damaged_size: &[u64],
    cache: &FrozenMap<CacheKey, Box<i64>>,
) -> i64 {
    let damaged_signature = build_damage_signature(&condition_record);

    // if damaged_signature.len() > contiguous_damaged_size.len() {
    //     return 0;
    // }

    if !zip(damaged_signature.iter(), contiguous_damaged_size.iter())
        .rev()
        .all(|(d, c)| contains(*d, *c))
    {
        // println!(
        //     "failed: {:?} {:?}    {}",
        //     contiguous_damaged_size,
        //     &damaged_signature,
        //     render_states(&condition_record)
        // );
        return 0;
    }

    if damaged_signature.len() == contiguous_damaged_size.len()
        && damaged_signature
            .last()
            .is_some_and(|d| matches!(d, Damage::Exact(_)))
    {
        // println!(
        //     "passed: {:?} {:?}    {}",
        //     contiguous_damaged_size,
        //     &damaged_signature,
        //     render_states(&condition_record)
        // );
        return 1;
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
                let operational_damage_signature =
                    build_damage_signature(&operational_condition_record);

                let mut damaged_condition_record = condition_record.clone();
                damaged_condition_record[i] = State::Damaged;
                let damaged_damage_signature = build_damage_signature(&damaged_condition_record);

                return cached_compute_possible_arrangements(
                    operational_condition_record,
                    contiguous_damaged_size,
                    cache,
                    CacheKey {
                        damage_signature: operational_damage_signature,
                    },
                ) + cached_compute_possible_arrangements(
                    damaged_condition_record,
                    contiguous_damaged_size,
                    cache,
                    CacheKey {
                        damage_signature: damaged_damage_signature,
                    },
                );
            }
        }
    }

    return 0;
}

pub fn part2(input: &str, repeats: usize) -> Result<i64> {
    let parsed = parse(input, repeats)?;
    Ok(parsed
        .iter()
        .map(|(c, d)| {
            dbg!(compute_possible_arrangements(
                c.clone(),
                d,
                &FrozenMap::new()
            ))
        })
        .sum())
}

#[cfg(test)]
mod part2_tests {
    use super::*;
    use indoc::indoc;
    use seq_macro::seq;

    #[test]
    fn example_5x() {
        let input = indoc! {r#"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"#};
        assert_eq!(part2(input, 5).expect("part2 should return Ok"), 525152);
    }

    #[test]
    fn example_1x() {
        let input = indoc! {r#"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"#};
        assert_eq!(part2(input, 1).expect("part2 should return Ok"), 21);
    }

    #[test]
    fn example_line1() {
        let input = indoc! {r#"
???.### 1,1,3
"#};
        assert_eq!(part2(input, 1).expect("part2 should return Ok"), 1);
    }

    #[test]
    fn example_line3() {
        let input = indoc! {r#"
?#?#?#?#?#?#?#? 1,3,1,6
"#};
        assert_eq!(part2(input, 0).expect("part2 should return Ok"), 1);
    }

    //     #[test]
    //     fn input_line4() {
    //         let input = indoc! {r#"
    // ?#?????##????#?? 1,9
    // "#};
    //         assert_eq!(part2(input, 0).expect("part2 should return Ok"), 3);
    //     }

    //     #[test]
    //     fn input_line11() {
    //         let input = indoc! {r#"
    // ?????????#?. 2,1,3
    // "#};
    //         assert_eq!(part2(input, 0).expect("part2 should return Ok"), 16);
    //     }

    // generate a test case for specific line in input, and assert that the part2() result matches the
    // part1 result when repeat is 0

    macro_rules! generate_test_case {
        ($line_number:literal) => {
            paste::paste! {
                #[test]
                fn [<input_line $line_number>]() {
                    let input = include_str!("../input.txt");
                    let expected = include_str!("../part_1_results.txt");
                    let input = input.lines().nth($line_number - 1).unwrap();
                    let expected = expected.lines().nth($line_number - 1).unwrap();
                    let input = format!("{}\n", input);
                    let expected = expected.parse::<i64>().unwrap();
                    dbg!(&input);
                    assert_eq!(part2(&input, 0).expect("part2 should return Ok"), expected);
                }
            }
        };
    }

    // call generate_test_case! for each line in input.txt
    seq!(N in 1..=1000 {
        generate_test_case!(N);
    });
}
