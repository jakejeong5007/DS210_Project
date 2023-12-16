mod data_loader;
mod graph_analysis;
use graph_analysis::Graph;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub fn save_data_to_file<T: ToString>(data: &[T], file_path: &str) -> Result<(), io::Error> {
    let path = Path::new(file_path);
    let mut file = File::create(&path)?;

    for item in data {
        writeln!(file, "{}", item.to_string())?;
    }

    Ok(())
}

fn main() {
    // Load the data
    let edges = data_loader::load_data("facebook_large/musae_facebook_edges.csv")
        .expect("Failed to load data");

    // Create the graph
    let mut graph = Graph::new();
    for (source, target) in edges {
        graph.add_edge(source, target);
    }

    // Perform clustering
    let clusters = graph_analysis::connected_components(&graph);
    println!("Clusters: {:?}", clusters);

    println!("Next");

    // Calculate closeness centrality
    let closeness_centrality = graph_analysis::closeness_centrality(&graph);
    println!("Closeness Centrality: {:?}", closeness_centrality);

    // Calculate and print the degree distribution
    let degree_distribution = graph_analysis::calculate_degree_distribution(&graph);
    for (degree, count) in degree_distribution.iter() {
        println!("Degree {}: {} nodes", degree, count);
    }

    // Print the graph summary
    graph_analysis::print_graph_summary(&graph);

    let centrality_str = closeness_centrality.iter()
        .enumerate()
        .map(|(index, &value)| format!("Node {}: {}", index, value))
        .collect::<Vec<String>>();

    match save_data_to_file(&centrality_str, "closeness_centrality.txt") {
        Ok(_) => println!("Closeness centrality saved successfully."),
        Err(e) => eprintln!("Failed to save closeness centrality: {}", e),
    }

    let degree_distribution_str = degree_distribution.iter()
        .map(|(degree, count)| format!("Degree {}: {} nodes", degree, count))
        .collect::<Vec<String>>();

    match save_data_to_file(&degree_distribution_str, "degree_distribution.txt") {
        Ok(_) => println!("Degree distribution saved successfully."),
        Err(e) => eprintln!("Failed to save degree distribution: {}", e),
    }

    let clusters_str = clusters.iter()
        .enumerate()
        .map(|(index, component)| format!("Component {}: {:?}", index, component))
        .collect::<Vec<String>>();

    match save_data_to_file(&clusters_str, "clusters.txt") {
        Ok(_) => println!("Clusters saved successfully."),
        Err(e) => eprintln!("Failed to save clusters: {}", e),
    }


}

#[cfg(test)]
mod connected_components_tests {
    use super::*;

    #[test]
    fn test_simple_graph() {
        let mut graph = Graph::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(3, 4);

        let components = connected_components(&graph);
        assert_eq!(components.len(), 2);
        assert!(components.contains(&vec![0, 1, 2]));
        assert!(components.contains(&vec![3, 4]));
    }

    #[test]
    fn test_single_node_graph() {
        let mut graph = Graph::new();
        graph.add_node(0);

        let components = connected_components(&graph);
        assert_eq!(components.len(), 1);
        assert_eq!(components[0], vec![0]);
    }
}

#[cfg(test)]
mod closeness_centrality_tests {
    use super::*;

    #[test]
    fn test_linear_graph() {
        let mut graph = Graph::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);

        let centrality = closeness_centrality(&graph);
        let expected_values = vec![0.5, 0.6666667, 0.6666667, 0.5];
        assert_eq!(centrality.len(), 4);
        for (calculated, &expected) in centrality.iter().zip(expected_values.iter()) {
            assert!((calculated - expected).abs() < 1e-6);
        }
    }

    #[test]
    fn test_star_graph() {
        let mut graph = Graph::new();
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(0, 3);
        graph.add_edge(0, 4);

        let centrality = closeness_centrality(&graph);
        let expected_values = vec![1.0, 0.5, 0.5, 0.5, 0.5];
        assert_eq!(centrality.len(), 5);
        for (calculated, &expected) in centrality.iter().zip(expected_values.iter()) {
            assert!((calculated - expected).abs() < 1e-6);
        }
    }
}
