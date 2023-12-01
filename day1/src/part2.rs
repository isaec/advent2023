use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input));
}

fn parse(s: &str) -> i32 {
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => s.parse::<i32>().expect("is a number"),
    }
}

pub fn part2(input: &str) -> String {
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let end_re = Regex::new(r".+(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    input
        .lines()
        .map(|line| {
            // get all the numbers in the line
            dbg!(line);
            let first = re.find(line).unwrap().as_str();
            let last = end_re
                .captures(line)
                .map(|m| m.get(1).unwrap().as_str())
                .unwrap_or(first);
            dbg!(format!("{}{}", parse(first), parse(last)))
        })
        .fold(0, |acc, x| acc + x.parse::<i32>().unwrap())
        .to_string()
}

#[cfg(test)]
mod part2_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#};
        assert_eq!(part2(input), "281");
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), "");
    }
}
