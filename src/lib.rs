//! Graphs

#![allow(incomplete_features)]
#![feature(adt_const_params)]

pub mod undirected_graph;
pub mod undirected_sparse_graph;

pub use undirected_graph::UndirectedGraph;
pub use undirected_sparse_graph::UndirectedSparseGraph;

/// A trait representing a generic graph.
///
/// This trait provides methods to manipulate vertices and edges in the graph, as well as to query
/// information about them.
///
/// # Type Parameters
///
/// - `T`: The type of data stored in the vertices of the graph.
///
pub trait Graph<T: Clone>: Clone {
    /// Adds a new vertex to the graph with the specified data.
    /// Returns the ID of the newly added vertex.
    ///
    /// # Arguments
    ///
    /// - `data`: The data to store in the vertex.
    fn add_vertex(&mut self, data: T) -> usize;

    /// Removes the vertex with the specified ID from the graph.
    /// Also removes all edges connected to the vertex.
    ///
    /// # Arguments
    ///
    /// - `vertex_id`: The ID of the vertex to remove.
    fn remove_vertex(&mut self, vertex_id: usize);

    /// Adds an edge between two vertices in the graph.
    ///
    /// # Arguments
    ///
    /// - `vertex_id_1`: The ID of the first vertex to connect.
    /// - `vertex_id_2`: The ID of the second vertex to connect.
    fn add_edge(&mut self, vertex_id_1: usize, vertex_id_2: usize);

    /// Removes an edge between two vertices in the graph.
    ///
    /// Does nothing if the edge does not exist.

    /// # Arguments
    ///
    /// - `vertex_id_1`: The ID of the first vertex to disconnect.
    /// - `vertex_id_2`: The ID of the second vertex to disconnect.
    fn remove_edge(&mut self, vertex_id_1: usize, vertex_id_2: usize);

    /// Gets the data stored in a vertex with the specified ID.
    /// Returns `None` if the vertex does not exist in the graph.
    ///
    /// # Arguments
    ///
    /// - `vertex_id`: The ID of the vertex to get the data for.
    fn get_vertex_data(&self, vertex_id: usize) -> Option<T>;

    /// Sets the data stored in a vertex with the specified ID.
    ///
    /// Does nothing if the vertex does not exist.
    ///
    /// # Arguments
    ///
    /// - `vertex_id`: The ID of the vertex to set the data for.
    /// - `data`: The new data to store in the vertex.
    fn set_vertex_data(&mut self, vertex_id: usize, data: T);

    /// Determines whether two vertices in the graph are adjacent.
    ///
    /// Returns `true` if there is an edge between the two vertices, `false` if there is no edge or if the vertices don't exist.
    ///
    /// # Arguments
    ///
    /// - `vertex_id_1`: The ID of the first vertex to test.
    /// - `vertex_id_2`: The ID of the second vertex to test.
    fn is_adjacent(&self, vertex_id_1: usize, vertex_id_2: usize) -> bool;

    /// Gets the IDs of all vertices connected to a given vertex.
    ///
    /// Returns a vector containing the IDs of all vertices that share an edge with the given vertex.
    ///
    /// # Arguments
    ///
    /// - `vertex_id`: The ID of the vertex to get the neighbors of.
    fn get_neighbors(&self, vertex_id: usize) -> Vec<usize>;
}

/// A trait representing a weighted graph.
pub trait WeightedGraph<T: Clone, W: Clone>: Graph<T> {
    /// Get the weight of the edge between two vertices.
	/// Returns the weight of the edge between the two vertices, if the edge exists. Otherwise, returns `None`.
    ///
    /// # Arguments
    ///
    /// * `vertex_id_1` - The ID of the first vertex.
    /// * `vertex_id_2` - The ID of the second vertex.
    fn get_edge_weight(&self, vertex_id_1: usize, vertex_id_2: usize) -> Option<W>;

    /// Set the weight of the edge between two vertices.
    ///
    /// # Arguments
    ///
    /// * `vertex_id_1` - The ID of the first vertex.
    /// * `vertex_id_2` - The ID of the second vertex.
    /// * `weight` - The weight to set.
    fn set_edge_weight(&mut self, vertex_id_1: usize, vertex_id_2: usize, weight: W);
}