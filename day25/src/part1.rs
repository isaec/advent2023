use std::sync::{mpsc::Sender, Arc};

use miette::Result;
use miette_pretty::Pretty;
use parse::{Grid, QuickRegex, Tile};
use petgraph::{
    algo,
    dot::{Config, Dot},
    graphmap::GraphMap,
    Undirected,
};
use rand::seq::SliceRandom;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input).unwrap());
}

fn parse(input: &str) -> Result<Vec<(&str, Vec<&str>)>> {
    input
        .lines()
        .map(|l| {
            let (comp, connections) = l.split_once(": ").pretty()?;
            let connections = connections.split(" ").collect();
            Ok((comp, connections))
        })
        .collect()
}

pub fn part1(input: &str) -> Result<i64> {
    let parsed = parse(input)?;
    // dbg!(&parsed);
    let mut graph: GraphMap<&str, (), Undirected> = GraphMap::new();
    for (comp, connections) in parsed {
        for connection in connections {
            graph.add_edge(comp, connection, ());
        }
    }

    let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    // std::fs::write("graph.dot", format!("{:?}", dot)).pretty()?;

    let cuts = [("rks", "kzh"), ("dgt", "tnz"), ("ddc", "gqm")];
    for (a, b) in cuts.iter() {
        graph.remove_edge(a, b);
    }

    assert_eq!(algo::connected_components(&graph), 2);

    let (a, b) = cuts[0];

    let count_connected_nodes = |start: &str| {
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(start);
        while let Some(node) = queue.pop_front() {
            if visited.contains(node) {
                continue;
            }
            visited.insert(node);
            for neighbor in graph.neighbors(node) {
                queue.push_back(neighbor);
            }
        }
        visited.len()
    };

    Ok(count_connected_nodes(a) as i64 * count_connected_nodes(b) as i64)
}

#[cfg(test)]
mod part1_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
"#};
        assert_eq!(part1(input).expect("part1 should return Ok"), 54);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input).expect("part1 should return Ok"), 0);
    }
}
