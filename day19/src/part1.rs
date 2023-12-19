use std::collections::HashMap;

use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use parse::{Grid, QuickRegex, Tile};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input).unwrap());
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Debug, Clone, Copy)]
enum PartCategory {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone)]
enum Compare {
    LT,
    GT,
}

#[derive(Debug, Clone)]
enum WorkflowStep {
    Compare(PartCategory, Compare, u64, String),
    Jump(String),
}

type Workflow = Vec<WorkflowStep>;

fn parse_category(input: &str) -> PartCategory {
    match input {
        "x" => PartCategory::X,
        "m" => PartCategory::M,
        "a" => PartCategory::A,
        "s" => PartCategory::S,
        _ => unreachable!(),
    }
}

fn parse(input: &str) -> Result<(Vec<Part>, HashMap<String, Workflow>)> {
    let (workflows, parts) = input.split_once("\n\n").pretty()?;

    let parts = parts
        .lines()
        .map(|line| {
            let (x, m, a, s) = line
                .get_digits()
                .unwrap()
                .iter()
                .map(|d| *d as u64)
                .collect_tuple()
                .unwrap();
            Part { x, m, a, s }
        })
        .collect_vec();

    let workflows = workflows
        .lines()
        .map(|line| {
            let (name, details) = line.split_once('{').unwrap();
            let details = details.trim_end_matches('}');
            let details = details
                .split(',')
                .map(|detail| {
                    let split = detail.split_once(':');
                    match split {
                        None => WorkflowStep::Jump(detail.to_string()),
                        Some((condition, jump)) => {
                            if condition.contains('>') {
                                let (category, value) = condition.split_once('>').unwrap();
                                let category = parse_category(category);
                                let value = value.parse().unwrap();
                                WorkflowStep::Compare(
                                    category,
                                    Compare::GT,
                                    value,
                                    jump.to_string(),
                                )
                            } else if condition.contains('<') {
                                let (category, value) = condition.split_once('<').unwrap();
                                let category = parse_category(category);
                                let value = value.parse().unwrap();
                                WorkflowStep::Compare(
                                    category,
                                    Compare::LT,
                                    value,
                                    jump.to_string(),
                                )
                            } else {
                                unreachable!()
                            }
                        }
                    }
                })
                .collect_vec();

            (name.to_string(), details)
        })
        .collect();

    Ok((parts, workflows))
}

pub fn part1(input: &str) -> Result<i64> {
    let parsed = parse(input)?;
    dbg!(parsed);
    Ok(0)
}

#[cfg(test)]
mod part1_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"#};
        assert_eq!(part1(input).expect("part1 should return Ok"), 19114);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input).expect("part1 should return Ok"), 0);
    }
}
