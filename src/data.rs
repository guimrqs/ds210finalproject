use petgraph::algo::astar;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Step 4
use std::collections::HashSet;


#[derive(Debug, Deserialize, Clone)]
pub struct Vertex {
    pub id: u64,
}

#[derive(Debug, Deserialize)]
pub struct Edge {
    pub source: u64,
    pub target: u64,
}

pub fn build_graph(data_path: &str) -> Graph<Vertex, ()> {
    let mut graph = Graph::<Vertex, ()>::new();
    let mut id_map: HashMap<u64, NodeIndex> = HashMap::new();

    for entry in std::fs::read_dir(data_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "edges" {
                let file = File::open(&path).unwrap();
                let reader = BufReader::new(file);

                for line in reader.lines() {
                    let line = line.unwrap();
                    let ids: Vec<u64> = line
                        .split_whitespace()
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect();

                    let source = *id_map
                        .entry(ids[0])
                        .or_insert_with(|| graph.add_node(Vertex { id: ids[0] }));
                    let target = *id_map
                        .entry(ids[1])
                        .or_insert_with(|| graph.add_node(Vertex { id: ids[1] }));

                    graph.add_edge(source, target, ());
                }
            }
        }
    }
    graph
}

pub fn shortest_path(graph: &Graph<Vertex, ()>, start_id: u64, end_id: u64) -> Option<(u64, Vec<NodeIndex>)> {
    let node_indices: Vec<NodeIndex> = graph.node_indices().collect();
    let start_index = node_indices.iter().find(|&&ni| graph[ni].id == start_id)?;
    let end_index = node_indices.iter().find(|&&ni| graph[ni].id == end_id)?;

    let result = astar(graph, *start_index, |finish| finish == *end_index, |_| 1, |_| 0);
    result.map(|(cost, path)| (cost, path))
}

pub fn print_first_n_vertices(graph: &Graph<Vertex, ()>, n: usize) {
    let node_indices: Vec<NodeIndex> = graph.node_indices().collect();
    let count = n.min(graph.node_count());

    for ni in node_indices.into_iter().take(count) {
        println!("Vertex ID: {}", graph[ni].id);
    }
}

pub fn compute_degree_centrality(graph: &Graph<Vertex, ()>) -> Vec<f64> {
    let n = graph.node_count() as f64;
    graph
        .node_indices()
        .map(|ni| graph.edges(ni).count() as f64 / (n - 1.0))
        .collect()
}

pub fn print_first_n_vertices_with_centrality(
    graph: &Graph<Vertex, ()>,
    centrality: &[f64],
    n: usize,
) {
    let node_indices: Vec<NodeIndex> = graph.node_indices().collect();
    let count = n.min(graph.node_count());

    for (i, ni) in node_indices.into_iter().take(count).enumerate() {
        println!("Vertex ID: {}, Centrality: {:.6}", graph[ni].id, centrality[i]);
    }
}

// Step 4

pub fn charikar_densest_subgraph(graph: &Graph<Vertex, ()>) -> f64 {
    let mut max_density = 0.0;
    let mut remaining_graph = Graph::<Vertex, ()>::new();
for node in graph.node_indices() {
    remaining_graph.add_node(graph[node].clone());
}
for edge in graph.raw_edges() {
    remaining_graph.add_edge(NodeIndex::new(edge.source().index()), NodeIndex::new(edge.target().index()), ());
}
    let mut remaining_vertices = graph.node_indices().collect::<HashSet<_>>();

    while !remaining_vertices.is_empty() {
        let density = remaining_graph.edge_count() as f64 / remaining_graph.node_count() as f64;

        if density > max_density {
            max_density = density;
        }

        let min_degree_vertex = remaining_vertices
            .iter()
            .min_by_key(|&&ni| remaining_graph.edges(ni).count())
            .copied()
            .unwrap();

        remaining_graph.remove_node(min_degree_vertex);
        remaining_vertices.remove(&min_degree_vertex);
    }

    max_density
}

pub fn to_adjacency_matrix(graph: &Graph<Vertex, ()>) -> HashMap<(NodeIndex, NodeIndex), f64> {
    let mut adjacency_matrix = HashMap::new();
    let node_indices: Vec<NodeIndex> = graph.node_indices().collect();

    for ni in &node_indices {
        for nj in &node_indices {
            let weight = if graph.contains_edge(*ni, *nj) {
                1.0
            } else {
                f64::INFINITY
            };
            adjacency_matrix.insert((*ni, *nj), weight);
        }
    }

    adjacency_matrix
}

pub fn single_linkage_clustering(graph: &Graph<Vertex, ()>, num_clusters: usize) -> Vec<Vec<NodeIndex>> {
    let mut clusters: Vec<Vec<NodeIndex>> = graph.node_indices().map(|ni| vec![ni]).collect();
    let adjacency_matrix = to_adjacency_matrix(graph);

    while clusters.len() > num_clusters {
        let mut min_distance = f64::INFINITY;
        let mut merge_pair = (0, 0);

        for i in 0..clusters.len() {
            for j in (i + 1)..clusters.len() {
                for ni in &clusters[i] {
                    for nj in &clusters[j] {
                        let distance = *adjacency_matrix.get(&(*ni, *nj)).unwrap_or(&f64::INFINITY);
                        if distance < min_distance {
                            min_distance = distance;
                            merge_pair = (i, j);
                        }
                    }
                }
            }
        }

        let (i, j) = merge_pair;
        let cluster_j = clusters.remove(j);
        clusters[i].extend(cluster_j.into_iter());
    }

    clusters
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_path() {
        let test_data_path = "/Users/guimarques/Desktop/data";
        let graph = build_graph(test_data_path);
        let start_id = 16287561; 
        let end_id = 77007853; 
        let expected_distance = 2;
        let expected_path = vec![16287561, 18668992, 77007853]; // Replace with the actual node IDs in the expected path 

        match shortest_path(&graph, start_id, end_id) {
            Some((distance, path)) => {
                assert_eq!(distance, expected_distance);
                assert_eq!(
                    path.iter().map(|ni| graph[*ni].id).collect::<Vec<_>>(),
                    expected_path
                );
            }
            None => {
                panic!("No path found between {} and {}.", start_id, end_id);
            }
        }
    }
}