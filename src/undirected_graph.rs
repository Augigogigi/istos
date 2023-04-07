use super::Graph;

/// This is NOT a multigraph, but it DOES support loop edges.
/// Uses an adjacency matrix.
/// Use this over a sparse graph if there are many of edges.
#[derive(Clone, Debug)]
pub struct UndirectedGraph<T: Clone> {
	vertices: Vec<(usize, T)>,
	edges: Vec<bool>,
	next_id: usize,
}

impl<T: Clone> UndirectedGraph<T> {
	/// Create a new UndirectedGraph.
	pub fn new() -> Self {
		Self {
			vertices: vec![],
			edges: vec![],
			next_id: 0,
		}
	}

	/// Utility function to access the edges. (they're stored in a 1D array)
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
		let id: usize = self.next_id;
		let size: usize = self.vertices.len();

		for i in 0..=size {
			self.edges.insert(size * i, false);
		}

		self.vertices.push((id, data));
		self.next_id += 1;

		id
	}

	fn remove_vertex(&mut self, vertex_id: usize) {
		let Some(pos) = self.get_index_from_id(vertex_id) else { return; };
		let size: usize = self.vertices.len();

		for i in (0..size).rev() {
			self.edges.remove(self.index_vector_with_coords(pos, i));
		}

		self.vertices.remove(pos);
	}

	fn add_edge(&mut self, vertex_id_1: usize, vertex_id_2: usize) {
		let Some(pos_1) = self.get_index_from_id(vertex_id_1) else { return; };
		let Some(pos_2) = self.get_index_from_id(vertex_id_2) else { return; };
		let index: usize = self.index_vector_with_coords(pos_1, pos_2);
		self.edges[index] = true;
	}

	fn remove_edge(&mut self, vertex_id_1: usize, vertex_id_2: usize) {
		let Some(pos_1) = self.get_index_from_id(vertex_id_1) else { return; };
		let Some(pos_2) = self.get_index_from_id(vertex_id_2) else { return; };
		let index: usize = self.index_vector_with_coords(pos_1, pos_2);
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
		let Some(pos_1) = self.get_index_from_id(vertex_id_1) else { return false; };
		let Some(pos_2) = self.get_index_from_id(vertex_id_2) else { return false; };
		let index: usize = self.index_vector_with_coords(pos_1, pos_2);
		self.edges[index]
	}

	fn get_neighbors(&self, vertex_id: usize) -> Vec<usize> {
		let mut res = Vec::new();
		for i in 0..self.vertices.len() {
			let other_id = self.vertices[i].0;
			if self.is_adjacent(vertex_id, other_id) {
				res.push(other_id);
			}
		}
		res
	}
}


/// This is NOT a multigraph, but it DOES support loop edges.
/// Uses an adjacency list.
/// Use this over a regular graph if there are few edges.
#[derive(Clone, Debug)]
pub struct UndirectedSparseGraph<T: Clone> {
	vertices: Vec<(usize, T)>,
	edges: Vec<(usize, usize)>,
	next_id: usize,
}

impl<T: Clone> UndirectedSparseGraph<T> {
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
		let id: usize = self.next_id;

		self.vertices.push((id, data));
		self.next_id += 1;

		id
	}

	fn remove_vertex(&mut self, vertex_id: usize) {
		self.vertices.retain(|x| x.0 != vertex_id);
	}

	fn add_edge(&mut self, vertex_id_1: usize, vertex_id_2: usize) {
		self.edges.push((vertex_id_1, vertex_id_2));
	}

	fn remove_edge(&mut self, vertex_id_1: usize, vertex_id_2: usize) {
		self.edges.retain(|&x| x != (vertex_id_1, vertex_id_2) || x != (vertex_id_2, vertex_id_1));
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
		for i in 0..self.vertices.len() {
			let other_id = self.vertices[i].0;
			if self.is_adjacent(vertex_id, other_id) {
				res.push(other_id);
			}
		}
		res
	}
}
