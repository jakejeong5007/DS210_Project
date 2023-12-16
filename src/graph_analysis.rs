// A simple graph structure
pub struct Graph {
    edges: Vec<(u32, u32)>,
    nodes: Vec<u32>,
}

impl Graph {
    // Initialize a new Graph
    pub fn new() -> Graph {
        Graph {
            edges: Vec::new(),
            nodes: Vec::new(),
        }
    }

    // Add a node to the graph
    pub fn add_node(&mut self, node: u32) {
        if !self.nodes.contains(&node) {
            self.nodes.push(node);
        }
    }

    // Add an edge to the graph
    pub fn add_edge(&mut self, src: u32, dest: u32) {
        self.edges.push((src, dest));
        self.add_node(src);
        self.add_node(dest);
    }

    // Get neighbors of a node
    pub fn neighbors(&self, node: u32) -> Vec<u32> {
        self.edges.iter()
            .filter_map(|&(src, dest)| {
                if src == node { Some(dest) }
                else if dest == node { Some(src) }
                else { None }
            })
            .collect()
    }
}

// Find connected components
pub fn connected_components(graph: &Graph) -> Vec<Vec<u32>> {
    let mut visited = vec![false; graph.nodes.len()];
    let mut components = Vec::new();

    for &node in &graph.nodes {
        if !visited[node as usize] {
            let mut component = Vec::new();
            dfs(node, &graph, &mut visited, &mut component);
            components.push(component);
        }
    }

    components
}

// Depth-first search to find connected components
fn dfs(node: u32, graph: &Graph, visited: &mut Vec<bool>, component: &mut Vec<u32>) {
    visited[node as usize] = true;
    component.push(node);

    for &neigh in &graph.neighbors(node) {
        if !visited[neigh as usize] {
            dfs(neigh, graph, visited, component);
        }
    }
}

// Calculate closeness centrality
pub fn closeness_centrality(graph: &Graph) -> Vec<f64> {
    graph.nodes.iter().map(|&node| {
        let total_distance: u32 = graph.nodes.iter()
            .filter(|&&other| other != node)
            .map(|&other| shortest_path_length(node, other, graph))
            .sum();
        let n = graph.nodes.len() as f64;
        (n - 1.0) as f64 / total_distance as f64
    }).collect()
}

// Function to find the shortest path length between two nodes
fn shortest_path_length(src: u32, dest: u32, graph: &Graph) -> u32 {
    if src == dest {
        return 0;
    }

    let mut visited = vec![false; graph.nodes.len()];
    let mut queue = std::collections::VecDeque::new();
    let mut distance = 0;

    visited[src as usize] = true;
    queue.push_back(src);

    while let Some(node) = queue.pop_front() {
        distance += 1;
        for &neighbor in &graph.neighbors(node) {
            if neighbor == dest {
                return distance;
            }
            if !visited[neighbor as usize] {
                visited[neighbor as usize] = true;
                queue.push_back(neighbor);
            }
        }
    }

    distance
}

pub fn calculate_degree_distribution(graph: &Graph) -> std::collections::HashMap<u32, u32> {
    let mut degree_distribution = std::collections::HashMap::new();

    for node in &graph.nodes {
        let degree = graph.neighbors(*node).len() as u32;
        *degree_distribution.entry(degree).or_insert(0) += 1;
    }

    degree_distribution
}

pub fn print_graph_summary(graph: &Graph) {
    let num_nodes = graph.nodes.len();
    let num_edges = graph.edges.len();

    println!("Graph Summary:");
    println!("Number of nodes: {}", num_nodes);
    println!("Number of edges: {}", num_edges);

}