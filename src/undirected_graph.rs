use super::Graph;

/// A graph data structure with undirected edges, implemented using an adjacency matrix.
///
/// A `UndirectedGraph` consists of a set of vertices and a set of edges connecting those vertices. Vertices are
/// identified by a unique `usize` identifier, and have some associated data of type `T`. Edges are
/// represented using an adjacency matrix, which stores a boolean value indicating whether there is an edge between
/// each pair of vertices.
///
/// # Example
///
/// ```
/// use istos::{Graph, UndirectedGraph};
///
/// let mut graph: UndirectedGraph<()> = UndirectedGraph::new();
///
/// // Add some vertices and edges
/// let v1 = graph.add_vertex(());
/// let v2 = graph.add_vertex(());
/// let v3 = graph.add_vertex(());
///
/// graph.add_edge(v1, v2);
/// graph.add_edge(v2, v3);
/// graph.add_edge(v3, v1);
///
/// // Remove an edge
/// graph.remove_edge(v1, v2);
///
/// // Print out the graph
/// println!("{:?}", graph);
/// ```
#[derive(Clone, Debug)]
pub struct UndirectedGraph<T: Clone> {
	vertices: Vec<(usize, T)>, // A vector of vertex IDs and associated data
	edges: Vec<bool>, // An adjacency matrix representing the edges between vertices
	next_id: usize, // The ID to assign to the next added vertex
}

impl<T: Clone> UndirectedGraph<T> {
	/// Create a blank UndirectedGraph.
	pub fn new() -> Self {
		Self {
			vertices: vec![],
			edges: vec![],
			next_id: 0,
		}
	}

	/// This function maps the (x, y) coordinates of a 2D matrix onto a 1D vector.
	/// The mapping is done in a way that accounts for the fact that the adjacency matrix
	/// of an undirected graph is symmetrical.
	#[inline]
	fn index_vector_with_coords(&self, x: usize, y: usize) -> usize {
		if x <= y {
			(self.vertices.len() * 2 - x - 1) * x / 2 + y
		} else {
			(self.vertices.len() * 2 - y - 1) * y / 2 + x
		}
	}

	/// Utility function to find the index of a vertex from an id.
	#[inline]
	fn get_index_from_id(&self, id: usize) -> Option<usize> {
		self.vertices.iter().position(|x| x.0 == id)
	}
}

impl<T: Clone> Graph<T> for UndirectedGraph<T> {
	fn add_vertex(&mut self, data: T) -> usize {
		// Get the next available vertex ID
		let id: usize = self.next_id;
		
		// Insert a new row and column in the adjacency matrix for the new vertex
		let size: usize = self.vertices.len();
		for i in 0..=size {
			self.edges.insert(size * i, false);
		}
		
		// Add the new vertex to the vertices vector with its associated data
		self.vertices.push((id, data));
		
		// Increment the next available vertex ID
		self.next_id += 1;
		
		// Return the ID of the new vertex
		id
	}

	fn remove_vertex(&mut self, vertex_id: usize) {
		// Find the index of the vertex to be removed.
		let Some(pos) = self.get_index_from_id(vertex_id) else { return; };

		// Remove all edges connected to the vertex.
		let size: usize = self.vertices.len();
		for i in (0..size).rev() {
			self.edges.remove(self.index_vector_with_coords(pos, i));
		}

		// Remove the vertex.
		self.vertices.remove(pos);
	}

	fn add_edge(&mut self, vertex_id_1: usize, vertex_id_2: usize) {
		// Get the positions of the vertices in the vertex list
		let Some(pos_1) = self.get_index_from_id(vertex_id_1) else { return; };
		let Some(pos_2) = self.get_index_from_id(vertex_id_2) else { return; };

		// Calculate the index of the corresponding edge in the `edges` vector
		let index: usize = self.index_vector_with_coords(pos_1, pos_2);

		// Set the value to true to indicate the presence of an edge
		self.edges[index] = true;
	}

	fn remove_edge(&mut self, vertex_id_1: usize, vertex_id_2: usize) {
		// Get the positions of the vertices in the vertex list
		let Some(pos_1) = self.get_index_from_id(vertex_id_1) else { return; };
		let Some(pos_2) = self.get_index_from_id(vertex_id_2) else { return; };

		// Calculate the index of the corresponding edge in the `edges` vector
		let index: usize = self.index_vector_with_coords(pos_1, pos_2);

		// Set the value to false to indicate the absence of an edge
		self.edges[index] = false;
	}

	fn get_vertex_data(&self, vertex_id: usize) -> Option<T> {
		Some(self.vertices.iter().find(|&x| x.0 == vertex_id)?.1.clone())
	}

	fn set_vertex_data(&mut self, vertex_id: usize, data: T) {
		let Some(pos) = self.get_index_from_id(vertex_id) else { return; };
		self.vertices[pos].1 = data;
	}

	fn is_adjacent(&self, vertex_id_1: usize, vertex_id_2: usize) -> bool {
		// Get the positions of the vertices in the vertex list
		let Some(pos_1) = self.get_index_from_id(vertex_id_1) else { return false; };
		let Some(pos_2) = self.get_index_from_id(vertex_id_2) else { return false; };

		// Calculate the index of the corresponding edge in the `edges` vector
		let index: usize = self.index_vector_with_coords(pos_1, pos_2);

		// Find the value of the edge at the index
		self.edges[index]
	}

	fn get_neighbors(&self, vertex_id: usize) -> Vec<usize> {
		let mut res = Vec::new();

		// Iterate through all vertices to find neighbors of the given vertex
		for i in 0..self.vertices.len() {
			let other_id = self.vertices[i].0;
			if self.is_adjacent(vertex_id, other_id) {
				res.push(other_id);
			}
		}
		
		res
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_add_vertex() {
		let mut graph: UndirectedGraph<usize> = UndirectedGraph::new();
		let v1 = graph.add_vertex(1);
		let v2 = graph.add_vertex(2);

		assert_eq!(graph.vertices.len(), 2);
		assert_eq!(graph.get_vertex_data(v1), Some(1));
		assert_eq!(graph.get_vertex_data(v2), Some(2));
	}

	#[test]
	fn test_remove_vertex() {
		let mut graph: UndirectedGraph<usize> = UndirectedGraph::new();
		let v1 = graph.add_vertex(1);
		let v2 = graph.add_vertex(2);
		let v3 = graph.add_vertex(3);

		graph.add_edge(v1, v2);
		graph.add_edge(v2, v3);

		graph.remove_vertex(v2);

		assert_eq!(graph.vertices.len(), 2);
		assert_eq!(graph.get_vertex_data(v1), Some(1));
		assert_eq!(graph.get_vertex_data(v2), None);
		assert_eq!(graph.get_vertex_data(v3), Some(3));
		assert_eq!(graph.is_adjacent(v1, v2), false);
		assert_eq!(graph.is_adjacent(v2, v3), false);
	}

	#[test]
	fn test_add_edge() {
		let mut graph: UndirectedGraph<usize> = UndirectedGraph::new();
		let v1 = graph.add_vertex(1);
		let v2 = graph.add_vertex(2);

		graph.add_edge(v1, v2);

		assert_eq!(graph.is_adjacent(v1, v2), true);
		assert_eq!(graph.is_adjacent(v2, v1), true);
	}

	#[test]
	fn test_remove_edge() {
		let mut graph: UndirectedGraph<usize> = UndirectedGraph::new();
		let v1 = graph.add_vertex(1);
		let v2 = graph.add_vertex(2);
		let v3 = graph.add_vertex(3);

		graph.add_edge(v1, v2);
		graph.add_edge(v2, v3);

		graph.remove_edge(v1, v2);

		assert_eq!(graph.is_adjacent(v1, v2), false);
		assert_eq!(graph.is_adjacent(v2, v1), false);
		assert_eq!(graph.is_adjacent(v2, v3), true);
	}

	#[test]
	fn test_get_vertex_data() {
		let mut graph: UndirectedGraph<usize> = UndirectedGraph::new();
		let v1 = graph.add_vertex(1);
		let v2 = graph.add_vertex(2);

		assert_eq!(graph.get_vertex_data(v1), Some(1));
		assert_eq!(graph.get_vertex_data(v2), Some(2));
		assert_eq!(graph.get_vertex_data(999), None);
	}

	#[test]
	fn test_set_vertex_data() {
		let mut graph: UndirectedGraph<usize> = UndirectedGraph::new();
		let v1 = graph.add_vertex(1);
		let v2 = graph.add_vertex(2);

		graph.set_vertex_data(v1, 3);

		assert_eq!(graph.get_vertex_data(v1), Some(3));
		assert_eq!(graph.get_vertex_data(v2), Some(2));
	}

	#[test]
	fn test_is_adjacent() {
		let mut graph: UndirectedGraph<()> = UndirectedGraph::new();
		let v1 = graph.add_vertex(());
		let v2 = graph.add_vertex(());
		let v3 = graph.add_vertex(());

		assert!(!graph.is_adjacent(v1, v2));
		assert!(!graph.is_adjacent(v2, v1));
		assert!(!graph.is_adjacent(v1, v3));
		assert!(!graph.is_adjacent(v3, v1));
		assert!(!graph.is_adjacent(v2, v3));
		assert!(!graph.is_adjacent(v3, v2));

		graph.add_edge(v1, v2);

		assert!(graph.is_adjacent(v1, v2));
		assert!(graph.is_adjacent(v2, v1));
		assert!(!graph.is_adjacent(v1, v3));
		assert!(!graph.is_adjacent(v3, v1));
		assert!(!graph.is_adjacent(v2, v3));
		assert!(!graph.is_adjacent(v3, v2));

		graph.add_edge(v2, v3);

		assert!(graph.is_adjacent(v1, v2));
		assert!(graph.is_adjacent(v2, v1));
		assert!(!graph.is_adjacent(v1, v3));
		assert!(!graph.is_adjacent(v3, v1));
		assert!(graph.is_adjacent(v2, v3));
		assert!(graph.is_adjacent(v3, v2));
	}

	#[test]
	fn test_get_neighbors() {
		let mut graph: UndirectedGraph<()> = UndirectedGraph::new();
		let v1 = graph.add_vertex(());
		let v2 = graph.add_vertex(());
		let v3 = graph.add_vertex(());

		assert_eq!(graph.get_neighbors(v1), vec![]);
		assert_eq!(graph.get_neighbors(v2), vec![]);
		assert_eq!(graph.get_neighbors(v3), vec![]);

		graph.add_edge(v1, v2);

		assert_eq!(graph.get_neighbors(v1), vec![v2]);
		assert_eq!(graph.get_neighbors(v2), vec![v1]);
		assert_eq!(graph.get_neighbors(v3), vec![]);

		graph.add_edge(v2, v3);

		assert_eq!(graph.get_neighbors(v1), vec![v2]);
		assert_eq!(graph.get_neighbors(v2), vec![v1, v3]);
		assert_eq!(graph.get_neighbors(v3), vec![v2]);
	}
}
