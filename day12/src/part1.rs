use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use parse::QuickRegex;
use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input).unwrap());
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
            let condition_record = condition_record
                .chars()
                .map(|c| match c {
                    '?' => State::Unknown,
                    '.' => State::Operational,
                    '#' => State::Damaged,
                    _ => unreachable!(),
                })
                .collect();
            let contiguous_damaged_size = contiguous_damaged_size.get_digits()?;
            Ok((condition_record, contiguous_damaged_size))
        })
        .try_collect()
}

fn generate_all_states(condition_record: &[State]) -> Vec<Vec<State>> {
    let mut states: Vec<Vec<State>> = vec![vec![]];
    for record_state in condition_record {
        match record_state {
            State::Operational | State::Damaged => {
                for state in &mut states {
                    state.push(*record_state);
                }
            }
            State::Unknown => {
                let mut new_states = vec![];
                for base_state in &states {
                    let mut state = base_state.clone();
                    state.push(State::Operational);
                    new_states.push(state);
                    let mut state = base_state.clone();
                    state.push(State::Damaged);
                    new_states.push(state);
                }
                states = new_states;
            }
        }
    }
    states.clone()
}

fn compute_possible_arrangements(
    condition_record: &[State],
    contiguous_damaged_size: &[i64],
) -> i64 {
    generate_all_states(condition_record)
        .into_iter()
        .par_bridge()
        .filter(|states| {
            let grouped = states.iter().group_by(|s| **s);
            // dbg!(states);

            let mut damaged_count = 0;
            for (state, group) in grouped.into_iter() {
                // dbg!(state);
                if state == State::Damaged {
                    let expected_size = contiguous_damaged_size.get(damaged_count);
                    if let Some(expected_size) = expected_size {
                        if group.count() != *expected_size as usize {
                            return false;
                        }
                        damaged_count += 1;
                    } else {
                        return false;
                    }
                }
            }
            damaged_count == contiguous_damaged_size.len()
        })
        .count() as i64
}

pub fn part1(input: &str) -> Result<i64> {
    let parsed = parse(input)?;
    Ok(parsed
        .iter()
        .map(|(c, d)| dbg!(compute_possible_arrangements(c, d)))
        .sum())
}

#[cfg(test)]
mod part1_tests {
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
        assert_eq!(part1(input).expect("part1 should return Ok"), 21);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input).expect("part1 should return Ok"), 0);
    }
}
