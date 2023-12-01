fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input));
}

pub fn part2(input: &str) -> String {
    "".into()
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("../example.txt");
        assert_eq!(part2(input), "");
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), "");
    }
}
