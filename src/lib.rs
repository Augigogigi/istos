//! Graphs

#![allow(incomplete_features)]
#![feature(adt_const_params)]

pub mod undirected_graph;

pub trait Graph<T: Clone>: Clone {
	fn add_vertex(&mut self, data: T) -> usize;
	fn remove_vertex(&mut self, vertex_id: usize);

	fn add_edge(&mut self, vertex_id_1: usize, vertex_id_2: usize);
	fn remove_edge(&mut self, vertex_id_1: usize, vertex_id_2: usize);

	fn get_vertex_data(&self, vertex_id: usize) -> Option<T>;
	fn set_vertex_data(&mut self, vertex_id: usize, data: T);

	fn is_adjacent(&self, vertex_id_1: usize, vertex_id_2: usize) -> bool;
	fn get_neighbors(&self, vertex_id: usize) -> Vec<usize>;
}

pub trait WeightedGraph<T: Clone, W: Clone>: Graph<T> + Clone {
	fn get_edge_weight(&self, vertex_id_1: usize, vertex_id_2: usize) -> W;
	fn set_edge_weight(&mut self, vertex_id_1: usize, vertex_id_2: usize, weight: W);
}