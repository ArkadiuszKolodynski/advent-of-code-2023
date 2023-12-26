use rand::seq::SliceRandom;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Graph = HashMap<String, HashSet<String>>;
type Edge = (String, String);

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    node: String,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut graph = parse_input("../input.txt");

    let optimal_edges = find_optimal_edges(&graph, 1000);
    assert!(optimal_edges.is_some());

    let Some((edge1, edge2, edge3)) = optimal_edges else {
        panic!()
    };

    remove_edge(&mut graph, &edge1);
    remove_edge(&mut graph, &edge2);
    remove_edge(&mut graph, &edge3);
    let connected_components = find_connected_components(&graph);

    println!(
        "Product: {:?}",
        connected_components[0].len() * connected_components[1].len()
    );
}

fn dfs(node: &String, graph: &Graph, visited: &mut HashSet<String>) {
    visited.insert(node.clone());
    for neighbor in &graph[node] {
        if !visited.contains(neighbor) {
            dfs(neighbor, graph, visited);
        }
    }
}

fn find_connected_components(graph: &Graph) -> Vec<HashSet<String>> {
    let mut visited = HashSet::new();
    let mut components = Vec::new();
    for node in graph.keys() {
        if !visited.contains(node) {
            let mut component = HashSet::new();
            dfs(node, graph, &mut component);
            visited.extend(component.clone());
            components.push(component);
        }
    }
    components
}

fn find_optimal_edges(graph: &Graph, num_trials: usize) -> Option<(Edge, Edge, Edge)> {
    let mut rng = rand::thread_rng();
    let nodes: Vec<String> = graph.keys().cloned().collect();
    let mut edge_counts: HashMap<Edge, usize> = HashMap::new();

    for _ in 0..num_trials {
        let node_a = nodes.choose(&mut rng).unwrap();
        let node_b = nodes.choose(&mut rng).unwrap();
        if node_a == node_b {
            continue;
        }

        let path = find_path(graph, node_a, node_b);
        for edge in path {
            let sorted_edge = if edge.0 < edge.1 {
                (edge.0.clone(), edge.1.clone())
            } else {
                (edge.1.clone(), edge.0.clone())
            };
            *edge_counts.entry(sorted_edge).or_insert(0) += 1;
        }
    }

    let mut edges: Vec<_> = edge_counts.iter().collect();
    edges.sort_by(|a, b| b.1.cmp(a.1));

    if edges.len() < 3 {
        None
    } else {
        Some((edges[0].0.clone(), edges[1].0.clone(), edges[2].0.clone()))
    }
}

fn find_path(graph: &Graph, start: &String, end: &String) -> Vec<Edge> {
    let mut heap = BinaryHeap::new();
    let mut costs: HashMap<String, usize> = HashMap::new();
    let mut parents: HashMap<String, String> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();

    heap.push(State {
        cost: 0,
        node: start.clone(),
    });
    costs.insert(start.clone(), 0);

    while let Some(State { cost, node }) = heap.pop() {
        if node == *end {
            let mut path = Vec::new();
            let mut current = end;
            while current != start {
                let parent = parents.get(current).unwrap();
                path.push((parent.clone(), current.clone()));
                current = parent;
            }
            path.reverse();
            return path;
        }

        if !visited.insert(node.clone()) {
            continue;
        }

        for neighbor in &graph[&node] {
            let next = State {
                cost: cost + 1,
                node: neighbor.clone(),
            };
            if next.cost < *costs.get(neighbor).unwrap_or(&usize::MAX) {
                heap.push(next.clone());
                costs.insert(neighbor.clone(), next.cost);
                parents.insert(neighbor.clone(), node.clone());
            }
        }
    }

    Vec::new()
}

fn remove_edge(graph: &mut Graph, edge: &Edge) {
    graph.get_mut(&edge.0).unwrap().retain(|x| x != &edge.1);
    graph.get_mut(&edge.1).unwrap().retain(|x| x != &edge.0);
}

fn parse_input<P>(filename: P) -> Graph
where
    P: AsRef<Path>,
{
    let mut connections = HashMap::new();
    for line in read_lines(filename) {
        if let Ok(ip) = line {
            let (key, rest) = ip.split_once(": ").unwrap();
            let rest = rest
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            connections.insert(key.to_string(), rest);
        }
    }
    let mut graph = Graph::new();
    for (key, values) in connections {
        for value in values {
            graph
                .entry(key.clone())
                .or_insert(HashSet::new())
                .insert(value.clone());
            graph
                .entry(value.clone())
                .or_insert(HashSet::new())
                .insert(key.clone());
        }
    }

    graph
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename);
    let file = match file {
        Ok(file) => file,
        Err(e) => panic!("Error: {}", e),
    };
    io::BufReader::new(file).lines()
}
