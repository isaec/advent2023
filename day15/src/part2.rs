use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input));
}

fn hash_algorithm(input: &str) -> u64 {
    let mut value = 0;
    for c in input.chars() {
        let ascii_value = c as u64;
        value += ascii_value;
        value *= 17;
        value %= 256;
    }

    value
}

fn parse(input: &str) -> impl Iterator<Item = (&str, Option<u64>)> {
    input.split(',').map(|s| s.trim()).map(|s| {
        s.split_once('=')
            .map(|(l, r)| (l, r.parse::<u64>().ok()))
            .unwrap_or((&s[..s.len() - 1], None))
    })
}

pub fn part2(input: &str) -> u64 {
    let parsed = parse(input);
    let mut boxes: HashMap<u64, Vec<(&str, u64)>> = HashMap::new();
    for (label, val) in parsed {
        let hash = hash_algorithm(label);
        dbg!((label, val));
        match val {
            Some(val) => {
                let entry = boxes.entry(hash).or_insert_with(Vec::new);
                let mut entry = entry
                    .iter()
                    .map(|(l, v)| if *l == label { (*l, val) } else { (*l, *v) })
                    .collect_vec();
                if entry.iter().filter(|(l, _)| *l == label).count() == 0 {
                    entry.push((label, val));
                }
                boxes.insert(hash, entry);
            }
            None => {
                let entry = boxes.entry(hash).or_insert_with(Vec::new);
                let entry = entry
                    .iter()
                    .filter(|(l, _)| *l != label)
                    .copied()
                    .collect_vec();
                boxes.insert(hash, entry);
            }
        }
        // dbg!(&boxes);
    }

    boxes
        .iter()
        .map(|(k, v)| {
            let box_number = k + 1;
            v.iter()
                .enumerate()
                .map(|(i, (_, v))| {
                    let slot_number = i as u64 + 1;
                    box_number * slot_number * v
                })
                .sum::<u64>()
        })
        .sum::<u64>()
}

#[cfg(test)]
mod part2_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
"#};
        assert_eq!(part2(input), 145);
    }

    #[test]
    fn hash_example() {
        let input = "HASH";
        assert_eq!(hash_algorithm(input), 52);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 259333);
    }
}
