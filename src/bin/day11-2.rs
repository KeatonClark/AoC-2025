use std::{
	collections::{HashMap},
	hash::RandomState,
	io::{self, BufRead},
};

use petgraph::{algo::all_simple_paths, graph::DiGraph};
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use petgraph::visit::EdgeRef;

pub fn all_paths_memo(
    graph: &Graph<String, i32, petgraph::Directed>,
    start: NodeIndex,
    end: NodeIndex,
) -> Vec<Vec<NodeIndex>> {
    let mut memo: HashMap<NodeIndex, Vec<Vec<NodeIndex>>> = HashMap::new();
    let mut stack = Vec::new();
    let mut on_path = vec![false; graph.node_count()];

    fn dfs(
        graph: &Graph<String, i32, petgraph::Directed>,
        current: NodeIndex,
        end: NodeIndex,
        memo: &mut HashMap<NodeIndex, Vec<Vec<NodeIndex>>>,
        on_path: &mut [bool],
        stack: &mut Vec<NodeIndex>,
    ) -> Vec<Vec<NodeIndex>> {

        // If memoized, return cached
        if let Some(cached) = memo.get(&current) {
            return cached.clone();
        }

        stack.push(current);
        on_path[current.index()] = true;

        let mut result_paths = Vec::new();

        if current == end {
            result_paths.push(stack.clone());
        } else {
            for edge in graph.edges(current) {
                let next = edge.target();
                if !on_path[next.index()] {
                    for mut subpath in dfs(graph, next, end, memo, on_path, stack) {
                        result_paths.push(subpath);
                    }
                }
            }
        }

        // leave current
        on_path[current.index()] = false;
        stack.pop();

        // memoize
        memo.insert(current, result_paths.clone());
        result_paths
    }

    dfs(graph, start, end, &mut memo, &mut on_path, &mut stack)
}

fn main() {
	let mut graph = DiGraph::<String, i32>::new();
	let mut nodes = HashMap::new();
	io::stdin()
		.lock()
		.lines()
		.into_iter()
		.map(|line| {
			let line = line.expect("Could not read input");
			let (node, edges) = line.split_once(':').expect("Not ':' in input");
			let node = node.to_string();
			let node_idx = if let Some(node_idx) = nodes.get(&node) {
				*node_idx
			} else {
				let node_idx = graph.add_node(node.clone());
				nodes.insert(node, node_idx);
				node_idx
			};
			for other_node in edges.trim().split_whitespace() {
				let other_node = other_node.to_string();
				let other_node_idx = if let Some(other_node_idx) = nodes.get(&other_node) {
					*other_node_idx
				} else {
					let other_node_idx = graph.add_node(other_node.clone());
					nodes.insert(other_node, other_node_idx);
					other_node_idx
				};
				graph.add_edge(node_idx, other_node_idx, 1);
			}
		})
		.count();
	let start = nodes.get(&"svr".to_string()).unwrap();
	let end = nodes.get(&"fft".to_string()).unwrap();
	let paths = all_paths_memo(&graph, *start, *end);
	let a = paths.len();

	let start = nodes.get(&"fft".to_string()).unwrap();
	let end = nodes.get(&"dac".to_string()).unwrap();
	let paths = all_paths_memo(&graph, *start, *end);
	let b = paths.len();

	let start = nodes.get(&"dac".to_string()).unwrap();
	let end = nodes.get(&"out".to_string()).unwrap();
	let paths = all_paths_memo(&graph, *start, *end);
	let c = paths.len();
	println!("{}", a*b*c);
}
