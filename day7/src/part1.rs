use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use parse::QuickRegex;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input).unwrap());
}

fn get_val(c: &char) -> i64 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap() as i64,
    }
}

fn get_hand_type(hand: &Vec<char>) -> Result<i64> {
    let matches = hand
        .iter()
        .unique()
        .map(|c| hand.iter().filter(|c2| c == *c2).collect::<Vec<_>>())
        .sorted_by(|a, b| b.len().cmp(&a.len()))
        .collect::<Vec<_>>();
    if matches.first().pretty()?.len() == 5 {
        return Ok(5);
    } else if matches.first().pretty()?.len() == 4 {
        return Ok(4);
    } else if matches.first().pretty()?.len() == 3 {
        Ok(3)
    } else if matches.first().pretty()?.len() == 2 {
        if matches.get(1).pretty()?.len() == 2 {
            Ok(2)
        } else {
            Ok(1)
        }
    } else {
        Ok(0)
    }
}

fn parse(input: &str) -> Result<Vec<(Vec<char>, i64)>> {
    input
        .lines()
        .map(|l| l.get_matches("[\\w\\d]+").expect("should match"))
        .map(|m| {
            let chars = m[0].chars().collect::<Vec<_>>();
            Ok((chars, m[1].parse::<i64>().pretty()?))
        })
        .collect()
}

pub fn part1(input: &str) -> Result<i64> {
    let parsed = parse(input)?;
    dbg!(&parsed);
    Ok(parsed
        .iter()
        .sorted_by(|a, b| {
            let a_hand_type = get_hand_type(&a.0).expect("should get hand type");
            let b_hand_type = get_hand_type(&b.0).expect("should get hand type");
            if a_hand_type == b_hand_type {
                let a_vals: Vec<_> = a.0.iter().map(|c| get_val(c)).collect();
                let b_vals: Vec<_> = b.0.iter().map(|c| get_val(c)).collect();
                dbg!((&a_vals, &b_vals));
                dbg!(a_vals.cmp(&b_vals))
            } else {
                a_hand_type.cmp(&b_hand_type)
            }
        })
        .enumerate()
        .map(|(i, (hand, val))| {
            dbg!((i, hand, val));
            (i + 1) as i64 * val
        })
        .sum::<i64>())
}

#[cfg(test)]
mod part1_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#};
        assert_eq!(part1(input).expect("part1 should return Ok"), 6440);
    }

    #[test]
    fn five_of_a_kind() {
        let input = "AAAAA";
        assert_eq!(
            get_hand_type(&input.chars().collect()).expect("should get hand type"),
            5
        );
    }

    #[test]
    fn four_of_a_kind() {
        let input = "AA8AA";
        assert_eq!(
            get_hand_type(&input.chars().collect()).expect("should get hand type"),
            4
        );
    }

    #[test]
    fn three_of_a_kind() {
        let input = "TTT98";
        assert_eq!(
            get_hand_type(&input.chars().collect()).expect("should get hand type"),
            3
        );
    }

    #[test]
    fn two_pair_kind() {
        let input = "23432";
        assert_eq!(
            get_hand_type(&input.chars().collect()).expect("should get hand type"),
            2
        );
    }

    #[test]
    fn one_pair_kind() {
        let input = "A23A4";
        assert_eq!(
            get_hand_type(&input.chars().collect()).expect("should get hand type"),
            1
        );
    }

    #[test]
    fn high_card_kind() {
        let input = "A2345";
        assert_eq!(
            get_hand_type(&input.chars().collect()).expect("should get hand type"),
            0
        );
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input).expect("part1 should return Ok"), 0);
    }
}
