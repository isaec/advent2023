use std::error::Error;

use fancy_regex::Regex;
use miette::Result;
use miette_pretty::Pretty;

pub trait QuickRegex {
    fn get_groups(&self, regex: &str) -> Result<Vec<&str>>;
    fn get_match(&self, regex: &str) -> Result<&str>;
    fn get_matches(&self, regex: &str) -> Result<Vec<&str>>;
    fn get_matches_parsed<T: std::str::FromStr>(&self, regex: &str) -> Result<Vec<T>>
    where
        <T as std::str::FromStr>::Err: Error + Send + Sync + 'static;
    fn get_digits(&self) -> Result<Vec<i64>>;
}

impl QuickRegex for str {
    #[track_caller]
    fn get_groups(&self, regex: &str) -> Result<Vec<&str>> {
        let re = Regex::new(regex).pretty_msg(format!("regex `{regex}` instantiation failed"))?;
        let msg = format!("regex `{regex}` capture in \"{self}\" failed to match");
        let captures = re.captures(self).pretty_msg(&msg)?.pretty_msg(&msg)?;

        Ok(captures
            .iter()
            .skip(1)
            .map(|c| c.unwrap().as_str())
            .collect())
    }

    #[track_caller]
    fn get_match(&self, regex: &str) -> Result<&str> {
        let re = Regex::new(regex).pretty_msg(format!("regex `{regex}` instantiation failed"))?;
        let msg = format!("regex `{regex}` find in \"{self}\" failed to match");
        let found = re.find(self).pretty_msg(&msg)?.pretty_msg(&msg)?;
        Ok(found.as_str())
    }

    #[track_caller]
    fn get_matches(&self, regex: &str) -> Result<Vec<&str>> {
        let re = Regex::new(regex).pretty_msg(format!("regex `{regex}` instantiation failed"))?;
        let matches = re.find_iter(self).map(|m| m.unwrap().as_str()).collect();
        Ok(matches)
    }

    #[track_caller]
    fn get_matches_parsed<T: std::str::FromStr>(&self, regex: &str) -> Result<Vec<T>>
    where
        <T as std::str::FromStr>::Err: Error + Send + Sync + 'static,
    {
        let re = Regex::new(regex).pretty_msg(format!("regex `{regex}` instantiation failed"))?;
        let matches = re
            .find_iter(self)
            .map(|m| {
                let str = m.unwrap().as_str();
                str.parse::<T>().pretty_msg(format!("parsing \"{}\"", &str))
            })
            .collect::<Result<Vec<T>>>()?;
        Ok(matches)
    }

    #[track_caller]
    fn get_digits(&self) -> Result<Vec<i64>> {
        let re = r"(?:(?<!\d)-)?\d+";
        let matches = self.get_matches_parsed(re)?;
        Ok(matches)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_groups() {
        let input = "1-3 a: abcde";
        let groups = input.get_groups(r"(\d+)-(\d+) (\w): (\w+)");
        assert_eq!(groups.unwrap(), vec!["1", "3", "a", "abcde"]);
    }

    #[test]
    fn test_get_matches() {
        let input = "1 12 some words10 2";
        let matches = input.get_matches(r"\d+");
        assert_eq!(matches.unwrap(), vec!["1", "12", "10", "2"]);
    }

    #[test]
    fn test_get_match() {
        let input = "1 12 some words10 2";
        let matches = input.get_match(r"some \w+\d+");
        assert_eq!(matches.unwrap(), "some words10");
    }

    #[test]
    fn test_get_matches_parsed() {
        let input = "1 12 some words10 2";
        let matches = input.get_matches_parsed::<usize>(r"\d+");
        assert_eq!(matches.unwrap(), vec![1, 12, 10, 2]);
    }

    #[test]
    fn test_get_digits() {
        let input = "1 -10 3-3 4 words and +5";
        let digits = input.get_digits();
        assert_eq!(digits.unwrap(), vec![1, -10, 3, 3, 4, 5]);
    }

    #[test]
    fn test_aoc2023_day4_input() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let (winners, my_numbers) = input.split_once("|").expect("contains |");
        let winners = winners
            .get_match(r":.+")
            .expect("number section exists")
            .get_digits();
        let my_numbers = my_numbers.get_digits();

        assert_eq!(winners.unwrap(), vec![41, 48, 83, 86, 17]);
        assert_eq!(my_numbers.unwrap(), vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }
}
