use std::hash::Hash;

use priority_queue::PriorityQueue;

fn main() {
    let input = get_input();
    let max_path = find_max_path(&input).expect("Failed to find max path");
    println!("Max path: {}", max_path);
}

fn find_max_path(input: &[Vec<Node>]) -> Option<usize> {
    // Use A* algorithm to find the maximum path
    let mut queue = PriorityQueue::new();
    let num_rows = input.len();
    let start = input.first().unwrap().first().unwrap();
    let start_cost = start.value;
    let h = heuristic(start, start_cost, num_rows);
    queue.push(
        start,
        NodeState {
            cost: start_cost,
            heuristic: h,
        },
    );

    while let Some((node, state)) = queue.pop() {
        if node.row == num_rows - 1 {
            return Some(state.cost);
        }
        // Only time we have no neighbors is when we're at the bottom row
        let neighbors = get_neighbors(node, input).expect("Failed to get neighbors");
        for neighbor in neighbors.iter() {
            let cost = state.cost + neighbor.value;
            let heuristic = heuristic(neighbor, cost, num_rows);
            // If the neighbor is already in the queue and the cost is higher, skip it
            if let Some((_, prev_state)) = queue.get(neighbor)
                && prev_state.cost >= cost
            {
                continue;
            }
            queue.push(neighbor, NodeState { cost, heuristic });
        }
    }
    None
}

fn heuristic(node: &Node, node_cost: usize, num_rows: usize) -> usize {
    // Want to overestimate the cost (we're trying to find the max path)
    // So assume the remaining rows have the maximum value (99)
    (num_rows - node.row - 1) * 99 + node_cost
}

fn get_neighbors<'a>(node: &Node, input: &'a [Vec<Node>]) -> Option<&'a [Node]> {
    let row = node.row + 1;
    input.get(row)?.get(node.col..=node.col + 1)
}

#[derive(Debug, Clone)]
struct NodeState {
    cost: usize,
    heuristic: usize,
}

impl PartialEq for NodeState {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.heuristic == other.heuristic
    }
}

impl Eq for NodeState {}

impl PartialOrd for NodeState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NodeState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heuristic.cmp(&other.heuristic)
    }
}

#[derive(Debug, Clone)]
struct Node {
    row: usize,
    col: usize,
    value: usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col && self.value == other.value
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.col.hash(state);
        self.value.hash(state);
    }
}

fn get_input() -> Vec<Vec<Node>> {
    let input = include_str!("../input.txt");
    parse_input(input)
}

#[allow(dead_code)]
fn get_test_input() -> Vec<Vec<Node>> {
    let input = "3\n7 4\n2 4 6\n8 5 9 3";
    parse_input(input)
}

fn parse_input(input: &str) -> Vec<Vec<Node>> {
    let mut result = Vec::new();
    for (row, line) in input.lines().enumerate() {
        let row: Vec<Node> = line
            .split_whitespace()
            .enumerate()
            .map(|(col, s)| Node {
                row,
                col,
                value: s.parse().expect("Failed to parse"),
            })
            .collect();
        result.push(row);
    }
    result
}
