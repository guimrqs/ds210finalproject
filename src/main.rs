mod data;

use data::{build_graph, compute_degree_centrality};

fn main() {
    let dataset_path = "/Users/guimarques/Desktop/data"; // FOR GRADER: Change path when you download dataset
    let graph = build_graph(dataset_path);
    println!("Graph constructed with {} nodes and {} edges.", graph.node_count(), graph.edge_count());

    println!("First 10 vertices:");
    data::print_first_n_vertices(&graph, 10);

    let start_id = 16287561;
    let end_id = 77007853;

    match data::shortest_path(&graph, start_id, end_id) {
        Some((distance, path)) => {
            println!("Shortest path between {} and {} has a distance of {}.", start_id, end_id, distance);
            println!("Path: {:?}", path.iter().map(|ni| graph[*ni].id).collect::<Vec<_>>());
        }
        None => {
            println!("No path found between {} and {}.", start_id, end_id);
        }
    }

    let degree_centrality = compute_degree_centrality(&graph);
    println!("Degree centrality for the first 10 vertices:");
    data::print_first_n_vertices_with_centrality(&graph, &degree_centrality, 10);
    
    let max_density = data::charikar_densest_subgraph(&graph);
    println!("The density of the densest subgraph is {:.2}.", max_density);
    println!("Densest subgraph calculation completed.");

    let num_clusters = 3;
    let clusters = data::single_linkage_clustering(&graph, num_clusters);
    println!("Clusters using single-linkage clustering:");

    for (i, cluster) in clusters.iter().enumerate() {
        let vertex_ids: Vec<u64> = cluster.iter().map(|&ni| graph[ni].id).collect();
        println!("Cluster {}: {:?}", i + 1, vertex_ids);
        println!("Single linkage clustering completed.");
    }
}