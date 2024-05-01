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
    println!("Pennsylvania Network Data");

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
