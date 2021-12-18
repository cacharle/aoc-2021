use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::str::FromStr;

const INPUT_FILENAME: &str = "12/input.txt";

// #[derive(Debug)]
struct CaveGraph {
    edges: Vec<Vec<bool>>,
    nodes: Vec<String>,
}

// #[derive(Clone)]
// struct Cave {
//     name: String,
//     children: Vec<Box<Cave>>,
// }

impl fmt::Debug for CaveGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.nodes.len() {
            write!(
                f,
                "{:>5}: {}\n",
                self.nodes[i],
                self.edges[i]
                    .iter()
                    .map(|&v| if v { "x" } else { "." })
                    .collect::<Vec<&str>>()
                    .join(" ")
            )?;
        }
        Ok(())
    }
}

impl FromStr for CaveGraph {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let edge_pairs: Vec<(&str, &str)> = s
            .lines()
            .map(|l| l.split_once("-"))
            .collect::<Option<Vec<_>>>()
            .ok_or("no '-' in line")?;

        let mut nodes = Vec::new();
        for (from, to) in &edge_pairs {
            let from = from.to_string();
            let to = to.to_string();
            if !nodes.contains(&from) {
                nodes.push(from.to_string());
            }
            if !nodes.contains(&to) {
                nodes.push(to.to_string());
            }
        }

        let mut edges = vec![vec![false; nodes.len()]; nodes.len()];
        for (from, to) in &edge_pairs {
            let from_pos = nodes.iter().position(|n| n == from).unwrap();
            let to_pos = nodes.iter().position(|n| n == to).unwrap();
            edges[from_pos][to_pos] = true;
            edges[to_pos][from_pos] = true;
        }

        Ok(CaveGraph { edges, nodes })
    }
}

impl CaveGraph {
    fn is_small_cave(name: &str) -> bool {
        name == name.to_lowercase()
    }

    fn children(&self, name: &str) -> Vec<String> {
        let pos = self.nodes.iter().position(|n| n == name).unwrap();
        self.edges[pos]
            .iter()
            .zip(self.nodes.iter())
            .filter(|(&e, _)| e)
            .map(|(_, n)| n.clone())
            .collect()
    }
}

fn explore_part1(graph: &CaveGraph, name: &str, mut current_path: Vec<String>) -> Vec<Vec<String>> {
    current_path.push(name.to_string());
    if name == "end" {
        return vec![current_path];
    }
    let mut paths = Vec::new();
    for child_name in graph.children(name) {
        if CaveGraph::is_small_cave(&child_name) && current_path.contains(&child_name) {
            continue;
        }
        let child_paths = explore_part1(graph, &child_name, current_path.clone());
        paths.extend(child_paths);
    }
    paths
}

fn has_visited_small_cave_two_times(current_path: &[String]) -> bool {
    let mut counter = HashMap::new();
    for p in current_path {
        if CaveGraph::is_small_cave(p) {
            *counter.entry(p).or_insert(0) += 1;
        }
    }
    let mut small_caves_visited_twice = 0;
    for &c in counter.values() {
        if c >= 2 {
            small_caves_visited_twice += 1;
            if small_caves_visited_twice >= 2 {
                return true;
            }
        }
    }
    return false;
}

fn explore_part2(graph: &CaveGraph, name: &str, mut current_path: Vec<String>) -> Vec<Vec<String>> {
    if CaveGraph::is_small_cave(&name)
        && has_visited_small_cave_two_times(&current_path)
    {
        return vec![];
    }
    current_path.push(name.to_string());

    if name == "end" {
        return vec![current_path];
    }
    let mut paths = Vec::new();
    for child_name in graph.children(name) {
        if current_path.iter().filter(|&p| CaveGraph::is_small_cave(&child_name) && p == &child_name).count() > 1 {
            continue;
        }
        if child_name == "start" {
            continue;
        }
        let child_paths = explore_part2(graph, &child_name, current_path.clone());
        paths.extend(child_paths);
    }
    paths
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(INPUT_FILENAME)?;
    let graph: CaveGraph = file_content.parse()?;

    let paths = explore_part1(&graph, "start", vec![]);
    println!("part1 result: {}", paths.len());

    // PERF: part2 takes about 20s to solve with -O3
    let paths = explore_part2(&graph, "start", vec![]);
    println!("part2 result: {}", paths.len());
    Ok(())
}

// examples:
/*
start-A
start-b
A-c
A-b
b-d
A-end
b-end

dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc

fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
*/
