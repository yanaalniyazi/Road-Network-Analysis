use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;
use rand::seq::SliceRandom;


pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


pub fn read_graph_from_file(file_path: &str) -> Graph<i32, ()> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    println!("Pennsylvania Road Network Data");

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                if ip.starts_with('#') {
                    continue;
                }
                let vec: Vec<i32> = ip.split_whitespace().map(|s| s.parse().unwrap()).collect();
                let from = vec[0];
                let to = vec[1];
                graph.entry(from).or_insert_with(Vec::new).push(to);
            }
        }
    }


    let mut undirected_graph = Graph::<i32, ()>::new();
    for (from, to_vec) in &graph {
        let from_node = undirected_graph.add_node(*from);
        for &to in to_vec {
            let to_node = undirected_graph.add_node(to);
            undirected_graph.add_edge(from_node, to_node, ());
        }
    }

    undirected_graph
}


pub fn calculate_degree(graph: &Graph<i32, ()>) -> HashMap<NodeIndex, f64> {
    let mut degrees = HashMap::new();
    //let max_possible_degree = (graph.node_count() - 1) as f64;

    for node in graph.node_indices() {
        let degree = graph.neighbors(node).count() as f64;
        //let normalized_centrality = degree / max_possible_degree;
        degrees.insert(node, degree);
    }

    degrees
}


pub fn calculate_degree_centrality(graph: &Graph<i32, ()>) -> HashMap<NodeIndex, f64> {
    let mut degree_centrality = HashMap::new();
    let max_possible_degree = (graph.node_count() - 1) as f64;

    for node in graph.node_indices() {
        let degree = graph.neighbors(node).count() as f64;
        let normalized_centrality = degree / max_possible_degree;
        degree_centrality.insert(node, normalized_centrality);
    }

    degree_centrality
}


pub fn calculate_average_distance(graph: &Graph<i32, ()>, size: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let selected_nodes: Vec<_> = graph.node_indices().collect();
    let subset: Vec<_> = selected_nodes.choose_multiple(&mut rng, size).cloned().collect();

    let mut total_distance = 0;
    let mut total_pairs = 0;

    for &start_node in &subset {
        let distances = bfs_distances(graph, start_node);

        for (_end_node, distance) in distances {
            total_distance += distance;
            total_pairs += 1;
        }
    }


    if total_pairs > 0 {
        total_distance as f64 / total_pairs as f64
    } else {
        0.0
    }
}


pub fn bfs_distances(graph: &Graph<i32, ()>, source: NodeIndex) -> HashMap<NodeIndex, usize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back((source, 0));
    distances.insert(source, 0);

    //bfs
    while let Some((current_node, current_distance)) = queue.pop_front() {
        //go through each neighbor
        for neighbor in graph.neighbors(current_node) {
            // see if the neighbor has been visited
            if !distances.contains_key(&neighbor) {
                //increased dist
                let new_distance = current_distance + 1;
                queue.push_back((neighbor, new_distance));
                distances.insert(neighbor, new_distance);
            }
        }
    }
    distances
}


//tests to check the calculate_degree and calculate_degree_centrality functions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_degree_centrality() {
        let graph = read_graph_from_file("roadNet-PA.txt");
        let degree_centrality = calculate_degree_centrality(&graph);


        println!("{:?}", degree_centrality);
    }

    #[test]
    fn test_calculate_degree() {
        let graph = read_graph_from_file("roadNet-PA.txt");
        let degree = calculate_degree(&graph);

        println!("{:?}", degree);
    }
}
