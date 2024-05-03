use petgraph::graph::NodeIndex;
use std::collections::HashMap;
mod graph;

fn main() {
    let undirected_graph = graph::read_graph_from_file("roadNet-PA.txt");

    let degrees = graph::calculate_degree(&undirected_graph);

    let degree_centrality = graph::calculate_degree_centrality(&undirected_graph);
    
    top_nodes_centrality("Degree Centrality", &degree_centrality, 10);
    top_nodes_degrees("Degree", &degrees, 10);

    let avg_distance = graph::calculate_average_distance(&undirected_graph, 1000);
    println!("Average Distance: {:.2}", avg_distance);
    
    }

fn top_nodes_centrality(centrality_type: &str, centrality: &HashMap<NodeIndex, f64>, top_count: usize) {
    println!("Top 10 Nodes by {}: ", centrality_type);

    let mut sorted_nodes: Vec<_> = centrality.iter().collect();
    sorted_nodes.sort_by(|(_, &a), (_, &b)| b.partial_cmp(&a).unwrap());

    for (i, (&node, &value)) in sorted_nodes.iter().take(top_count).enumerate() {
        println!("{}. Node {}: {:.12}", i + 1, node.index(), value);
    }

    println!();
}


fn top_nodes_degrees(centrality_type: &str, centrality: &HashMap<NodeIndex, f64>, top_count: usize) {
    println!("Top 10 Nodes by {}: ", centrality_type);

    let mut sorted_nodes: Vec<_> = centrality.iter().collect();
    sorted_nodes.sort_by(|(_, &a), (_, &b)| b.partial_cmp(&a).unwrap());

    for (i, (&node, &value)) in sorted_nodes.iter().take(top_count).enumerate() {
        println!("{}. Node {}: {:.2}", i + 1, node.index(), value);
    }

    println!();
}
