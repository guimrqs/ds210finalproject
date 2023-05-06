use petgraph::graph::Graph;
use super::data::{build_graph, shortest_path, Vertex};

#[test]
fn test_shortest_path() {
    let dataset_path = "/Users/guimarques/Desktop/data";
    let graph = build_graph(dataset_path);

    let start_id = 16287561;
    let end_id = 77007853;

    let result = shortest_path(&graph, start_id, end_id);

    assert!(
        result.is_some(),
        "Expected to find a path between {} and {}",
        start_id,
        end_id
    );

    let (distance, path) = result.unwrap();
    assert_eq!(distance, 2, "Expected the shortest path distance to be 2");

    let expected_path = vec![start_id, 18668992, end_id];
    let actual_path: Vec<u64> = path.iter().map(|ni| graph[*ni].id).collect();

    assert_eq!(
        actual_path, expected_path,
        "Expected the shortest path to be {:?}, but got {:?}",
        expected_path, actual_path
    );
}
