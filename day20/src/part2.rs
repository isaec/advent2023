use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use petgraph::{graphmap::GraphMap, visit::IntoEdgeReferences, Directed};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input));
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Pulse {
    Low,
    High,
}

impl From<bool> for Pulse {
    fn from(b: bool) -> Self {
        match b {
            true => Pulse::High,
            false => Pulse::Low,
        }
    }
}

impl Pulse {
    fn flip(self) -> Self {
        match self {
            Pulse::Low => Pulse::High,
            Pulse::High => Pulse::Low,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Module {
    // %s
    FlipFlop(Pulse),
    // &s
    Conjunction(Option<HashMap<String, Pulse>>),
    // broadcaster
    Broadcaster,
}

/// The priority of an edge. Lower values are higher priority.
type Priority = u64;

fn parse(input: &str) -> (GraphMap<&str, Priority, Directed>, HashMap<&str, Module>) {
    let mut states = HashMap::new();
    let mut graph = GraphMap::new();
    for line in input.lines() {
        let (module, destinations) = line.split_once(" -> ").unwrap();
        let destinations = destinations.split(", ").collect::<Vec<_>>();
        let (name, module) = match module {
            s if s.starts_with('%') => (s.trim_start_matches('%'), Module::FlipFlop(Pulse::Low)),
            s if s.starts_with('&') => (s.trim_start_matches('&'), Module::Conjunction(None)),
            "broadcaster" => ("broadcaster", Module::Broadcaster),
            _ => unreachable!(),
        };
        states.insert(name, module);
        graph.add_node(name);
        for (priority, destination) in destinations.iter().enumerate() {
            graph.add_edge(name, destination, priority as u64);
        }
    }
    (graph, states)
}

/// Get the the names of the outgoing edges of a node, ordered by priority.
fn get_ordered_edges<'a>(
    graph: &'a GraphMap<&str, Priority, Directed>,
    name: &'a str,
) -> impl Iterator<Item = &'a str> {
    graph
        .edges(name)
        .sorted_by_key(|(_, _, priority)| *priority)
        .map(move |(_, to, _)| to)
}

pub fn part2(input: &str) -> u64 {
    let (graph, mut state_map) = parse(input);

    let mut presses = 0;
    loop {
        presses += 1;
        dbg!(presses);
        let mut stack = VecDeque::from(vec![("broadcaster", (Pulse::Low, "anon"))]);
        let mut rx_low_pulses = 0;
        let mut rx_high_pulses = 0;
        while let Some((name, (pulse, origin))) = stack.pop_front() {
            // dbg!((name, pulse, origin, &stack));
            if name == "output" {
                continue;
            }
            if name == "rx" {
                match pulse {
                    Pulse::Low => rx_low_pulses += 1,
                    Pulse::High => rx_high_pulses += 1,
                }
                continue;
            }
            let module_state = state_map.get_mut(name).unwrap();

            match module_state {
                Module::Conjunction(None) => {
                    let mut last_received = HashMap::new();
                    for (from, _, _) in graph.edge_references().filter(|(_, to, _)| *to == name) {
                        // dbg!((&module_state, from));
                        last_received.insert(from.to_string(), Pulse::Low);
                    }
                    // MAYBE EVIL
                    *module_state = Module::Conjunction(Some(last_received));
                }
                _ => {}
            }

            match (pulse, module_state) {
                (_, Module::Broadcaster) => {
                    for destination in get_ordered_edges(&graph, name) {
                        stack.push_back((destination, (pulse, name)));
                    }
                }
                (Pulse::High, Module::FlipFlop(_)) => {}
                (Pulse::Low, Module::FlipFlop(state)) => {
                    let new_state = state.flip();
                    let module_state = Module::FlipFlop(new_state);
                    state_map.insert(name, module_state);
                    for destination in get_ordered_edges(&graph, name) {
                        stack.push_back((destination, (new_state, name)));
                    }
                }
                (_, Module::Conjunction(state)) => {
                    let state = state.as_mut().unwrap();
                    state.insert(origin.to_string(), pulse);
                    // dbg!(&state);
                    let send = if state.values().all(|v| *v == Pulse::High) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };
                    for destination in get_ordered_edges(&graph, name) {
                        stack.push_back((destination, (send, name)));
                    }
                }
            }
        }
        if rx_low_pulses == 1 && rx_high_pulses == 0 {
            break;
        }
    }

    presses
}

#[cfg(test)]
mod part2_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn seb_example() {
        let input = indoc! {r#"
%nr -> mr
&sx -> zh
%rk -> dc, bl
%lx -> rs
%hx -> bl
%hp -> bj
%dk -> mr, lf
%hc -> xc
%bj -> vv, rd
&jt -> zh
&bl -> ks, kn, dc, hc, zk
&zh -> rx
%sp -> hz, bl
%rd -> vv, tp
%cg -> dk
%rg -> jl, pv
%jl -> js
%fb -> vv, zd
%gv -> lx
%lr -> vj, bl
%vz -> hc, bl
%kn -> bl, zk
%rj -> mr, nr
%cn -> pv, sb
%rs -> vv, hp
&mr -> qc, kb, gc, vl, bs, cg, lf
%rb -> qj
%sm -> bv, vv
%dh -> rg
%zk -> vz
%qj -> xs, pv
%ng -> ql, pv
%vj -> bl, sp
&kb -> zh
%sb -> pv
%vl -> mr, cz
%dc -> lr
%xc -> rk, bl
%cz -> cg, mr
%hz -> bl, hx
%xs -> pv, cn
%js -> ng
%cb -> mr, nc
%qb -> vv
%gc -> qc
%bv -> qb, vv
broadcaster -> kn, fb, ln, vl
%bs -> cb
%lf -> gc
%nc -> mr, rj
%ln -> pv, dh
%qc -> bs
&vv -> zd, jt, fb, hp, gv, lx
&ks -> zh
%ql -> rb
%tp -> sm, vv
&pv -> sx, dh, jl, ln, js, rb, ql
%zd -> gv
"#};
        assert_eq!(part2(input), 243081086866483);
    }
}
