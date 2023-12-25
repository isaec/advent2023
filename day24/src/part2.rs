use std::ops::RangeInclusive;

use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use parse::QuickRegex;
use z3::ast::Ast;
use z3::ast::Int;
use z3::ast::Real;
use z3::Config;
use z3::Context;
use z3::Solver;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input, 200000000000000..=400000000000000).unwrap());
}

type Triple = (i64, i64, i64);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Line {
    position: Triple,
    velocity: Triple,
}

fn parse(input: &str) -> Result<Vec<Line>> {
    input
        .lines()
        .map(|l| {
            let (position, velocity) = l.split_once('@').pretty()?;
            let position = position
                .get_digits()?
                .into_iter()
                .collect_tuple()
                .pretty()?;
            let velocity = velocity
                .get_digits()?
                .into_iter()
                .collect_tuple()
                .pretty()?;
            Ok(Line { position, velocity })
        })
        .collect()
}

pub fn part2(input: &str, _bound: RangeInclusive<i64>) -> Result<i64> {
    let parsed = parse(input)?;
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let throw_x = Real::new_const(&ctx, "throw_x");
    let throw_y = Real::new_const(&ctx, "throw_y");
    let throw_z = Real::new_const(&ctx, "throw_z");

    let throw_dx = Real::new_const(&ctx, "throw_dx");
    let throw_dy = Real::new_const(&ctx, "throw_dy");
    let throw_dz = Real::new_const(&ctx, "throw_dz");

    let r = |v| Real::from_int(&Int::from_i64(&ctx, v));

    macro_rules! assert_expr {
        (r($a1:expr) + r($b1:expr) * $t:ident == $a2:ident + $b2:ident * $t2:ident) => {
            solver.assert(
                &((r($a1) + r($b1) * $t.clone())._eq(&($a2.clone() + $b2.clone() * $t2.clone()))),
            );
        };
    }

    for (i, rock) in parsed.iter().enumerate().take(3) {
        let tn = Real::new_const(&ctx, format!("t{i}").as_str());
        solver.assert(&tn.ge(&r(0)));
        assert_expr!(r(rock.position.0) + r(rock.velocity.0) * tn == throw_x + throw_dx * tn);
        assert_expr!(r(rock.position.1) + r(rock.velocity.1) * tn == throw_y + throw_dy * tn);
        assert_expr!(r(rock.position.2) + r(rock.velocity.2) * tn == throw_z + throw_dz * tn);
    }

    solver.check();

    let model = solver.get_model().pretty()?;
    let out = model.eval(&(throw_x + throw_y + throw_z), true).pretty()?;
    let out = out.as_real().pretty()?;
    Ok(out.0 / out.1)
}

#[cfg(test)]
mod part2_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
"#};
        assert_eq!(part2(input, 7..=27).expect("part2 should return Ok"), 47);
    }
}
