use std::{collections::HashSet, fmt::Display};

use advent_of_code::util::iter::CountIf as _;
use itertools::Itertools;
use petgraph::{prelude::GraphMap, Undirected};

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u64> {
    let graph = parse_input(input);
    let graph_ref = &graph;

    let cliques2 = graph
        .all_edges()
        .map(|(from, to, _)| (from, to))
        .collect_vec();
    let cliques3 = cliques2
        .into_iter()
        .flat_map(|(from, to)| {
            graph_ref
                .neighbors(from)
                .filter(move |n| graph_ref.contains_edge(to, *n))
                .map(move |n| {
                    let mut c = [from, to, n];
                    c.sort();
                    c
                })
        })
        .collect::<HashSet<_>>();

    let result = cliques3.iter().count_if(|nodes| {
        nodes
            .iter()
            .any(|&computer| computer >= "ta".into() && computer <= "tz".into())
    });

    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<String> {
    let graph = parse_input(input);

    let maximal_cliques = bron_kerbosch(&graph);
    let max_clique = maximal_cliques
        .iter()
        .max_by_key(|clique| clique.len())
        .unwrap();
    let password = max_clique.iter().sorted().join(",");

    Some(password)
}

fn bron_kerbosch(graph: &GraphMap<Computer, (), Undirected>) -> Vec<HashSet<Computer>> {
    let mut max_cliques = Vec::new();
    let nodes = graph.nodes().collect();
    bron_kerbosch_inner(
        graph,
        &mut max_cliques,
        HashSet::new(),
        nodes,
        HashSet::new(),
    );
    max_cliques
}

fn bron_kerbosch_inner(
    graph: &GraphMap<Computer, (), Undirected>,
    max_cliques: &mut Vec<HashSet<Computer>>,
    r: HashSet<Computer>,
    mut p: HashSet<Computer>,
    mut x: HashSet<Computer>,
) {
    if p.is_empty() && x.is_empty() {
        max_cliques.push(r.clone());
        return;
    }

    while !p.is_empty() {
        let v = *p.iter().next().unwrap();
        let neighbors = graph.neighbors(v).collect();
        let sub_p = p.intersection(&neighbors).cloned().collect();
        let sub_x = x.intersection(&neighbors).cloned().collect();
        let mut sub_r = r.clone();
        sub_r.insert(v);
        bron_kerbosch_inner(graph, max_cliques, sub_r, sub_p, sub_x);
        p.remove(&v);
        x.insert(v);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Computer(u16);

impl From<&str> for Computer {
    fn from(s: &str) -> Self {
        Computer(
            (s.as_bytes()[0] - b'a') as u16 * 26
                + (s.as_bytes()[1] - b'a') as u16,
        )
    }
}

impl From<Computer> for String {
    fn from(v: Computer) -> Self {
        let a = (v.0 / 26) as u8 + b'a';
        let b = (v.0 % 26) as u8 + b'a';
        String::from_utf8([a, b].to_vec()).unwrap()
    }
}

impl Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from(*self))
    }
}

fn parse_input(input: &str) -> GraphMap<Computer, (), Undirected> {
    let mut graph = GraphMap::new();

    for line in input.lines() {
        let (from, to) = line.split_once("-").unwrap();
        graph.add_edge(from.into(), to.into(), ());
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_owned()));
    }
}
