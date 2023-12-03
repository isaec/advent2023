use miette::Result;

// add pretty and pretty_msg to Result to show the line number where the error occurred
use miette_pretty::Pretty;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input).unwrap());
}

pub fn part2(input: &str) -> Result<u64> {
    let num = "12".parse::<u64>().pretty_msg("parse to u64")?;
    Ok(num)
}

#[cfg(test)]
mod part2_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"

"#};
        dbg!(&input);
        assert_eq!(part2(input).expect("part2 should return Ok"), 0);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input).expect("part2 should return Ok"), 0);
    }
}
