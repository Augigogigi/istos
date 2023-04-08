use super::Graph;

/// The UndirectedSparseGraph struct represents an undirected sparse graph implemented
/// using a variant of an adjacency list. The graph consists of a set of vertices, each of which
/// has a unique usize identifier and some associated data of type T. The edges of the
/// graph are represented using a vector of pairs of vertex identifiers.
///
/// # Example
///
/// ```
/// use istos::{Graph, UndirectedSparseGraph};
///
/// let mut graph: UndirectedSparseGraph<()> = UndirectedSparseGraph::new();
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
pub struct UndirectedSparseGraph<T: Clone> {
	vertices: Vec<(usize, T)>, // A vector of vertex IDs and associated data
	edges: Vec<(usize, usize)>, // An list of the edges between vertices
	next_id: usize, // The ID to assign to the next added vertex
}

impl<T: Clone> UndirectedSparseGraph<T> {
	/// Create a blank UndirectedSparseGraph.
	pub fn new() -> Self {
		Self {
			vertices: vec![],
			edges: vec![],
			next_id: 0,
		}
	}
}

impl<T: Clone> Graph<T> for UndirectedSparseGraph<T> {
	fn add_vertex(&mut self, data: T) -> usize {
		// Get the next available vertex ID
		let id: usize = self.next_id;

		// Add the new vertex to the vertices vector with its associated data
		self.vertices.push((id, data));

		// Increment the next available vertex ID
		self.next_id += 1;

		// Return the ID of the new vertex
		id
	}

	fn remove_vertex(&mut self, vertex_id: usize) {
		self.vertices.retain(|x| x.0 != vertex_id);
		self.edges.retain(|&x| x.0 != vertex_id && x.1 != vertex_id);
	}

	fn add_edge(&mut self, vertex_id_1: usize, vertex_id_2: usize) {
		self.edges.push((vertex_id_1, vertex_id_2));
	}

	fn remove_edge(&mut self, vertex_id_1: usize, vertex_id_2: usize) {
		self.edges.retain(|&x| x != (vertex_id_1, vertex_id_2) && x != (vertex_id_2, vertex_id_1));
	}

	fn get_vertex_data(&self, vertex_id: usize) -> Option<T> {
		Some(self.vertices.iter().find(|&x| x.0 == vertex_id)?.1.clone())
	}

	fn set_vertex_data(&mut self, vertex_id: usize, data: T) {
		self.vertices.iter_mut().find(|x| x.0 == vertex_id).unwrap().1 = data;
	}

	fn is_adjacent(&self, vertex_id_1: usize, vertex_id_2: usize) -> bool {
		self.edges.contains(&(vertex_id_1, vertex_id_2)) || self.edges.contains(&(vertex_id_2, vertex_id_1))
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
		let mut graph: UndirectedSparseGraph<usize> = UndirectedSparseGraph::new();
		let v1 = graph.add_vertex(1);
		let v2 = graph.add_vertex(2);

		assert_eq!(graph.vertices.len(), 2);
		assert_eq!(graph.get_vertex_data(v1), Some(1));
		assert_eq!(graph.get_vertex_data(v2), Some(2));
	}

	#[test]
	fn test_remove_vertex() {
		let mut graph: UndirectedSparseGraph<usize> = UndirectedSparseGraph::new();
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
		let mut graph: UndirectedSparseGraph<usize> = UndirectedSparseGraph::new();
		let v1 = graph.add_vertex(1);
		let v2 = graph.add_vertex(2);

		graph.add_edge(v1, v2);

		assert_eq!(graph.edges.len(), 1);
		assert_eq!(graph.is_adjacent(v1, v2), true);
		assert_eq!(graph.is_adjacent(v2, v1), true);
	}

	#[test]
	fn test_remove_edge() {
		let mut graph: UndirectedSparseGraph<usize> = UndirectedSparseGraph::new();
		let v1 = graph.add_vertex(1);
		let v2 = graph.add_vertex(2);
		let v3 = graph.add_vertex(3);

		graph.add_edge(v1, v2);
		graph.add_edge(v2, v3);

		graph.remove_edge(v1, v2);

		assert_eq!(graph.edges.len(), 1);
		assert_eq!(graph.is_adjacent(v1, v2), false);
		assert_eq!(graph.is_adjacent(v2, v1), false);
		assert_eq!(graph.is_adjacent(v2, v3), true);
	}

	#[test]
	fn test_get_vertex_data() {
		let mut graph: UndirectedSparseGraph<usize> = UndirectedSparseGraph::new();
		let v1 = graph.add_vertex(1);
		let v2 = graph.add_vertex(2);

		assert_eq!(graph.get_vertex_data(v1), Some(1));
		assert_eq!(graph.get_vertex_data(v2), Some(2));
		assert_eq!(graph.get_vertex_data(999), None);
	}

	#[test]
	fn test_set_vertex_data() {
		let mut graph: UndirectedSparseGraph<usize> = UndirectedSparseGraph::new();
		let v1 = graph.add_vertex(1);
		let v2 = graph.add_vertex(2);

		graph.set_vertex_data(v1, 3);

		assert_eq!(graph.get_vertex_data(v1), Some(3));
		assert_eq!(graph.get_vertex_data(v2), Some(2));
	}

	#[test]
	fn test_is_adjacent() {
		let mut graph: UndirectedSparseGraph<()> = UndirectedSparseGraph::new();
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
		let mut graph: UndirectedSparseGraph<()> = UndirectedSparseGraph::new();
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
