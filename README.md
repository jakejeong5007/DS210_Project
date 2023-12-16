# Project Overview: Facebook Large Page-Page Network Analysis

## Introduction

This project focuses on analyzing the "Facebook Large Page-Page Network" dataset, leveraging the power of Rust for efficient data processing and analysis. The primary goal is to understand community formation and interaction within large-scale networks, particularly focusing on graph clustering and centrality measures. This understanding is crucial for comprehending social media dynamics and the inherent complexities of large-scale social networks.

## Components

The project is divided into three main components:

1. **Data Loader** (`data_loader.rs`): Responsible for loading and preprocessing the dataset. It reads the data file, performs necessary conversions, and ensures data integrity.

2. **Graph Analysis** (`graph_analysis.rs`): Contains algorithms for network analysis. It includes functions for graph clustering, calculating closeness and betweenness centrality, and additional utility functions like printing graph summaries and calculating degree distribution.

3. **Main Application** (`main.rs`): The central execution point of the project. It integrates the data loading and graph analysis components, orchestrating the flow from data input to analytical output.

## Running the Project

To run the project, follow these steps:

1. **Setup**: Ensure Rust and Cargo are installed on your system. Clone the project repository and navigate to the project directory.

2. **Data Preparation**: Place the "Facebook Large Page-Page Network" dataset in a known directory. The file should be in CSV format, representing the graph's edges.

3. **Configuration**: In `main.rs`, update the file path to point to your dataset.

4. **Execution**: Run the project using the command `cargo run`. Cargo will compile and execute the application.

5. **Testing**: Run unit tests using `cargo test` to validate the functionality of various components.

## Output and Analysis

The application will output several key pieces of information:

- **Graph Summary.txt**: Displays basic information like the number of nodes and edges in the graph.

- **Connected Components.txt**: Shows the identified clusters within the network. Each cluster represents a group of pages that are interconnected.

- **Centrality Measures.txt**: Outputs the closeness and betweenness centrality scores for each node, highlighting the most influential pages in the network.

- **Degree Distribution.txt**: Presents the distribution of node degrees across the network, offering insights into the network's structural properties.

## Conclusion

This Rust-based project provides a robust and efficient platform for analyzing large-scale networks, with a focus on social media structures. By examining community formation and key influencers within the Facebook Large Page-Page Network, the project offers valuable insights into the dynamics of social networks. The modular design ensures ease of use and flexibility, allowing for further expansion and customization to accommodate additional analytical needs.
