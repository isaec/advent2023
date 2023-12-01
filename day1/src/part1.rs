fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input));
}

pub fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let nums = line
                .split("")
                .filter(|c| "0123456789".contains(c))
                .filter(|c| !c.is_empty())
                .collect::<Vec<_>>();

            if nums.len() == 0 {
                return "0".to_string();
            }

            format!("{}{}", nums[0], nums[nums.len() - 1])
        })
        .fold(0, |acc, x| acc + x.parse::<i32>().unwrap())
        .to_string()
}

#[cfg(test)]
mod part1_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
        "#};
        assert_eq!(part1(input), "142");
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), "");
    }
}
