use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use parse::QuickRegex;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input).unwrap());
}

fn parse(input: &str) -> Result<Vec<Vec<i64>>> {
    input.lines().map(|l| l.get_digits()).collect()
}

fn take_difference(nums: &Vec<i64>) -> Vec<i64> {
    nums.iter().tuple_windows().map(|(a, b)| b - a).collect()
}

fn find_next(nums: Vec<i64>) -> i64 {
    let mut seqs = vec![nums];
    while !seqs
        .last()
        .expect("seqs should not be empty")
        .iter()
        .all(|n| *n == 0)
    {
        let last = seqs.last().unwrap();
        let next = take_difference(last);
        seqs.push(next);
    }

    // extrapolate
    let mut down = 0;
    for i in (0..seqs.len() - 1).rev() {
        let right = seqs[i].first().unwrap();
        dbg!(i);
        dbg!(down);
        dbg!(right);
        down = right - down;
        dbg!(down);
    }

    dbg!((&seqs, down));

    down
}

pub fn part2(input: &str) -> Result<i64> {
    let parsed = parse(input)?;
    Ok(parsed.iter().map(|nums| find_next(nums.to_vec())).sum())
}

#[cfg(test)]
mod part2_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#};
        assert_eq!(part2(input).expect("part2 should return Ok"), 2);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input).expect("part2 should return Ok"), 0);
    }
}
