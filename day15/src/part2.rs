use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use parse::QuickRegex;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input).unwrap());
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

fn parse(input: &str) -> Result<Vec<&str>> {
    Ok(input.split(',').map(|s| s.trim()).collect_vec())
}

pub fn part2(input: &str) -> Result<u64> {
    let parsed = parse(input)?;
    Ok(parsed
        .iter()
        .map(|s| dbg!(hash_algorithm(dbg!(s))))
        .sum::<u64>())
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
        assert_eq!(part2(input).expect("part2 should return Ok"), 1320);
    }

    #[test]
    fn hash_example() {
        let input = "HASH";
        assert_eq!(hash_algorithm(input), 52);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input).expect("part2 should return Ok"), 0);
    }
}
