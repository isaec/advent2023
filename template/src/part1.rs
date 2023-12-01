fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input));
}

pub fn part1(input: &str) -> String {
    "".into()
}

#[cfg(test)]
mod part1_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"

        "#};
        assert_eq!(part1(input), "");
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), "");
    }
}
