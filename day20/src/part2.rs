use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use num::{integer::lcm, Integer};
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
    let mut critical_presses: HashMap<&str, Vec<u64>> = HashMap::new();
    loop {
        presses += 1;
        if presses > 50_000 {
            break;
        }
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
            if name == "rm" && pulse == Pulse::High {
                critical_presses.entry(origin).or_default().push(presses);
                println!("origin: {origin}, presses: {presses}");
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

    critical_presses
        .iter()
        .map(|(_, presses)| {
            presses
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .last()
                .unwrap()
        })
        .reduce(|a, b| lcm(a, b))
        .unwrap()
}
