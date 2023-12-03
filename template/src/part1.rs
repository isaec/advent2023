use miette::Result;

// add pretty and pretty_msg to Result to show the line number where the error occurred
use miette_pretty::Pretty;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input).unwrap());
}

pub fn part1(input: &str) -> Result<u64> {
    let num = "12".parse::<u64>().pretty_msg("parse to u64")?;
    Ok(num)
}

#[cfg(test)]
mod part1_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"

"#};
        dbg!(&input);
        assert_eq!(part1(input).expect("part1 should return Ok"), 0);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input).expect("part1 should return Ok"), 0);
    }
}
