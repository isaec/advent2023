use std::{collections::HashMap, fmt::Debug};

use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input).unwrap());
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct RangeSet {
    from: u64,
    to: u64,
}

impl RangeSet {
    fn contains(&self, value: u64) -> bool {
        value >= self.from && value <= self.to
    }

    fn intersection(&self, other: &RangeSet) -> Option<RangeSet> {
        if self.from > other.to || self.to < other.from {
            None
        } else {
            Some(RangeSet {
                from: self.from.max(other.from),
                to: self.to.min(other.to),
            })
        }
    }

    fn len(&self) -> u64 {
        self.to - self.from + 1
    }

    fn subset_greater_than(&self, value: u64) -> Option<RangeSet> {
        if value >= self.to {
            None
        } else {
            Some(RangeSet {
                from: value + 1,
                to: self.to,
            })
        }
    }

    fn subset_less_than(&self, value: u64) -> Option<RangeSet> {
        if value <= self.from {
            None
        } else {
            Some(RangeSet {
                from: self.from,
                to: value - 1,
            })
        }
    }

    fn subset_greater_than_or_equal(&self, value: u64) -> Option<RangeSet> {
        if value > self.to {
            None
        } else {
            Some(RangeSet {
                from: value,
                to: self.to,
            })
        }
    }

    fn subset_less_than_or_equal(&self, value: u64) -> Option<RangeSet> {
        if value < self.from {
            None
        } else {
            Some(RangeSet {
                from: self.from,
                to: value,
            })
        }
    }

    fn partition_upper_inclusive(&self, value: u64) -> (Option<RangeSet>, Option<RangeSet>) {
        (
            self.subset_less_than(value),
            self.subset_greater_than_or_equal(value),
        )
    }

    fn partition_lower_inclusive(&self, value: u64) -> (Option<RangeSet>, Option<RangeSet>) {
        (
            self.subset_less_than_or_equal(value),
            self.subset_greater_than(value),
        )
    }
}

impl Debug for RangeSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RangeSet[{}, {}]", self.from, self.to)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct PartSet {
    x: RangeSet,
    m: RangeSet,
    a: RangeSet,
    s: RangeSet,
}

impl PartSet {
    fn copy_overwrite(&mut self, key: PartCategory, value: RangeSet) -> PartSet {
        match key {
            PartCategory::X => PartSet { x: value, ..*self },
            PartCategory::M => PartSet { m: value, ..*self },
            PartCategory::A => PartSet { a: value, ..*self },
            PartCategory::S => PartSet { s: value, ..*self },
        }
    }

    fn intersection(&self, other: &PartSet) -> Option<PartSet> {
        let x = self.x.intersection(&other.x)?;
        let m = self.m.intersection(&other.m)?;
        let a = self.a.intersection(&other.a)?;
        let s = self.s.intersection(&other.s)?;
        Some(PartSet { x, m, a, s })
    }

    fn count_combinations(&self) -> u64 {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

impl Part {
    fn sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
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
enum WorkflowResult {
    Accept,
    Reject,
    Jump(String),
}

fn parse_workflow_result(input: &str) -> WorkflowResult {
    match input {
        "A" => WorkflowResult::Accept,
        "R" => WorkflowResult::Reject,
        _ => WorkflowResult::Jump(input.to_string()),
    }
}

#[derive(Debug, Clone)]
enum WorkflowStep {
    Compare(PartCategory, Compare, u64, WorkflowResult),
    Do(WorkflowResult),
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

fn parse(input: &str) -> Result<HashMap<String, Workflow>> {
    let (workflows, _parts) = input.split_once("\n\n").pretty()?;

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
                        None => WorkflowStep::Do(parse_workflow_result(detail)),
                        Some((condition, result)) => {
                            if condition.contains('>') {
                                let (category, value) = condition.split_once('>').unwrap();
                                let category = parse_category(category);
                                let value = value.parse().unwrap();
                                WorkflowStep::Compare(
                                    category,
                                    Compare::GT,
                                    value,
                                    parse_workflow_result(result),
                                )
                            } else if condition.contains('<') {
                                let (category, value) = condition.split_once('<').unwrap();
                                let category = parse_category(category);
                                let value = value.parse().unwrap();
                                WorkflowStep::Compare(
                                    category,
                                    Compare::LT,
                                    value,
                                    parse_workflow_result(result),
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

    Ok(workflows)
}

// fn test_workflow(workflow: &Workflow, part: &Part) -> WorkflowResult {
//     for step in workflow {
//         match step {
//             WorkflowStep::Compare(category, compare, compare_value, result) => {
//                 let part_value = match category {
//                     PartCategory::X => part.x,
//                     PartCategory::M => part.m,
//                     PartCategory::A => part.a,
//                     PartCategory::S => part.s,
//                 };
//                 match compare {
//                     Compare::LT => {
//                         if part_value < *compare_value {
//                             return result.clone();
//                         }
//                     }
//                     Compare::GT => {
//                         if part_value > *compare_value {
//                             return result.clone();
//                         }
//                     }
//                 }
//             }
//             WorkflowStep::Do(result) => return result.clone(),
//         }
//     }
//     unreachable!()
// }

fn get_set_results(workflow: &Workflow) -> Vec<(PartSet, WorkflowResult)> {
    let mut results = vec![];
    let mut current = PartSet {
        x: RangeSet { from: 0, to: 4000 },
        m: RangeSet { from: 0, to: 4000 },
        a: RangeSet { from: 0, to: 4000 },
        s: RangeSet { from: 0, to: 4000 },
    };

    for step in workflow {
        match step {
            WorkflowStep::Compare(category, compare, compare_value, result) => {
                let part_value = match category {
                    PartCategory::X => current.x,
                    PartCategory::M => current.m,
                    PartCategory::A => current.a,
                    PartCategory::S => current.s,
                };
                match compare {
                    Compare::LT => {
                        let (lower, upper) = part_value.partition_upper_inclusive(*compare_value);
                        if let Some(lower) = lower {
                            results
                                .push((current.copy_overwrite(*category, lower), result.clone()));
                        }
                        if let Some(upper) = upper {
                            current = current.copy_overwrite(*category, upper);
                        } else {
                            return results;
                        }
                    }
                    Compare::GT => {
                        let (lower, upper) = part_value.partition_lower_inclusive(*compare_value);
                        if let Some(upper) = upper {
                            results
                                .push((current.copy_overwrite(*category, upper), result.clone()));
                        }
                        if let Some(lower) = lower {
                            current = current.copy_overwrite(*category, lower);
                        } else {
                            return results;
                        }
                    }
                }
            }
            WorkflowStep::Do(result) => {
                results.push((current, result.clone()));
                return results;
            }
        }
    }
    unreachable!()
}

// fn will_accept(workflows: &HashMap<String, Workflow>, part: &Part) -> bool {
//     let part = part;
//     let mut workflow = &workflows["in"];
//     loop {
//         match test_workflow(workflow, &part) {
//             WorkflowResult::Accept => return true,
//             WorkflowResult::Reject => return false,
//             WorkflowResult::Jump(name) => {
//                 workflow = &workflows[&name];
//             }
//         }
//     }
// }

pub fn part2(input: &str) -> Result<u64> {
    let workflows = parse(input)?;

    let workflows: HashMap<String, _> = workflows
        .iter()
        .map(|(name, workflow)| {
            let results = get_set_results(workflow);
            (name.clone(), results)
        })
        .collect();

    let mut part_sets = vec![(
        PartSet {
            x: RangeSet { from: 1, to: 4001 },
            m: RangeSet { from: 1, to: 4001 },
            a: RangeSet { from: 1, to: 4001 },
            s: RangeSet { from: 1, to: 4001 },
        },
        "in",
    )];

    let mut accepted = vec![];

    while let Some((part_set, to_be_applied)) = part_sets.pop() {
        dbg!(("pop", to_be_applied, &part_set));
        let results = &workflows[to_be_applied];
        for (new_part_set, result) in results {
            dbg!(("results", &new_part_set));
            let new_part_set = part_set.intersection(new_part_set);
            if let Some(new_part_set) = new_part_set {
                match result {
                    WorkflowResult::Accept => {
                        dbg!(("accept", to_be_applied, &new_part_set));
                        accepted.push(new_part_set);
                    }
                    WorkflowResult::Reject => {
                        dbg!(("reject", to_be_applied, &new_part_set));
                    }
                    WorkflowResult::Jump(name) => {
                        dbg!(("jump", name, &new_part_set));
                        part_sets.push((new_part_set, name));
                    }
                }
            }
        }
    }

    // dbg!(&accepted);

    Ok(accepted.iter().map(|part| part.count_combinations()).sum())
}

#[cfg(test)]
mod part2_tests {
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
        assert_eq!(
            part2(input).expect("part2 should return Ok"),
            167409079868000
        );
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input).expect("part2 should return Ok"), 0);
    }

    #[test]
    fn range_set_intersection_works() {
        let a = RangeSet { from: 0, to: 10 };
        let b = RangeSet { from: 5, to: 15 };
        let c = RangeSet { from: 0, to: 5 };
        let d = RangeSet { from: 10, to: 15 };
        let e = RangeSet { from: 5, to: 10 };
        let f = RangeSet { from: 0, to: 15 };
        let g = RangeSet { from: 5, to: 5 };
        let h = RangeSet { from: 15, to: 15 };

        assert_eq!(a.intersection(&b), Some(RangeSet { from: 5, to: 10 }));
        assert_eq!(a.intersection(&c), Some(RangeSet { from: 0, to: 5 }));
        assert_eq!(a.intersection(&d), Some(RangeSet { from: 10, to: 10 }));
        assert_eq!(a.intersection(&e), Some(RangeSet { from: 5, to: 10 }));
        assert_eq!(a.intersection(&f), Some(RangeSet { from: 0, to: 10 }));
        assert_eq!(a.intersection(&g), Some(RangeSet { from: 5, to: 5 }));
        assert_eq!(a.intersection(&h), None);
    }
}
