use miette::Result;
use miette_pretty::Pretty;
use parse::QuickRegex;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input).unwrap());
}

fn parse(input: &str) -> Result<Vec<&str>> {
    input.lines().map(|l| Ok(l)).collect()
}

pub fn part2(input: &str) -> Result<i64> {
    let parsed = parse(input)?;
    Ok(0)
}

#[cfg(test)]
mod part2_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"

"#};
        assert_eq!(part2(input).expect("part2 should return Ok"), 0);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input).expect("part2 should return Ok"), 0);
    }
}
