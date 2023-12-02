fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input));
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone)]
struct Game {
    number: usize,
    rounds: Vec<Vec<(usize, Color)>>,
}

impl Game {
    fn is_possible(&self) -> bool {
        for round in &self.rounds {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for (number, color) in round {
                match color {
                    Color::Red => red += number,
                    Color::Green => green += number,
                    Color::Blue => blue += number,
                }
            }
            if (red > 12) || (green > 13) || (blue > 14) {
                return false;
            }
        }
        true
    }
}

fn parse_input(input: &str) -> Vec<Game> {
    let mut games = Vec::new();
    for line in input.lines() {
        let mut parts = line.split(": ");
        let number = parts
            .next()
            .unwrap()
            .split(" ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let rounds = parts
            .next()
            .unwrap()
            .split("; ")
            .map(|round| {
                round
                    .split(", ")
                    .map(|card| {
                        let mut parts = card.split(" ");
                        let number = parts.next().unwrap().parse().unwrap();
                        let color = match parts.next().unwrap() {
                            "red" => Color::Red,
                            "green" => Color::Green,
                            "blue" => Color::Blue,
                            _ => unreachable!(),
                        };
                        (number, color)
                    })
                    .collect()
            })
            .collect();
        games.push(Game { number, rounds });
    }
    games
}

pub fn part1(input: &str) -> String {
    let games = parse_input(input);
    // The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
    games
        .iter()
        .filter(|game| game.is_possible())
        .fold(0, |acc, game| acc + game.number)
        .to_string()
}

#[cfg(test)]
mod part1_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#};
        assert_eq!(part1(input), "8");
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), "");
    }
}
