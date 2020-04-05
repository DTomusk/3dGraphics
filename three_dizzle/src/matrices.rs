pub struct vertex {
	leaving: edge,
	coords: (i32, i32, i32),
}

pub struct edge {
	twin: edge, 
	next: edge, 
	origin: vertex,
}

pub struct face {
	incident: edge,
}

pub struct matrix {
	columns: Vec<vector>,
}

pub struct vector {
	entries: Vec<i32>,
}