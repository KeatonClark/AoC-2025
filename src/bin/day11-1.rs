use std::{
	collections::HashMap,
	hash::RandomState,
	io::{self, BufRead},
};

use petgraph::{algo::all_simple_paths, graph::DiGraph};

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
	let start = nodes.get(&"you".to_string()).unwrap();
	let end = nodes.get(&"out".to_string()).unwrap();

	let paths = all_simple_paths::<Vec<_>, _, RandomState>(&graph, *start, *end, 1, None);
	println!("{}", paths.count());
}
