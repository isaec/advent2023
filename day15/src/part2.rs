use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input));
}

fn hash_algorithm(input: &str) -> usize {
    let mut value = 0;
    for c in input.chars() {
        let ascii_value = c as usize;
        value += ascii_value;
        value *= 17;
        value %= 256;
    }

    value
}

fn parse(input: &str) -> impl Iterator<Item = (&str, Option<usize>)> {
    input.split(',').map(|s| s.trim()).map(|s| {
        s.split_once('=')
            .map(|(l, r)| (l, r.parse::<usize>().ok()))
            .unwrap_or((&s[..s.len() - 1], None))
    })
}

pub fn part2(input: &str) -> usize {
    let parsed = parse(input);
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];
    for (label, val) in parsed {
        let hash = hash_algorithm(label);
        dbg!((label, val));
        match val {
            Some(val) => {
                if let Some(i) = boxes[hash].iter().position(|(l, _)| *l == label) {
                    boxes[hash][i] = (label, val);
                } else {
                    boxes[hash].push((label, val));
                }
            }
            None => {
                if let Some(i) = boxes[hash].iter().position(|(l, _)| *l == label) {
                    boxes[hash].remove(i);
                }
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(box_i, v)| {
            v.iter()
                .enumerate()
                .map(|(slot_i, (_, v))| (box_i + 1) * (slot_i + 1) * v)
                .sum::<usize>()
        })
        .sum::<usize>()
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
