pub struct Vertex {
	leaving: Edge,
	coords: (i32, i32, i32),
}

pub struct Edge {
	twin: Edge,
	next: Edge,
	origin: Vertex,
	incident: Face,
}

pub struct Face {
	incident: Edge,
}

pub struct Polyhedron {
	faces: Vec<Face>,
}

pub struct Matrix {
	columns: Vec<Vector>,
}

pub struct Vector {
	entries: Vec<i32>,
}

impl Vector {
	pub fn add(A:Vector, B:Vector) -> Result<Vector, &'static str> {
		if A.len() != B.len() {
			Err("Vectors of different lengths")
		} else {
			let C = vec![];
			// this isn't right
			for i in 0..A.len() {
				C.push(A[i]+B[i]);
			}
			Ok(C)
		}
	}

	pub fn sProduct(A:Vector, B:Vector) -> Result<i32, &'static str> {
		if A.len() != B.len() {
			Err("Vectors of different lengths")
		} else {
			let mut C = 0;
			for i in 0..A.len() {
				C += A[i]*B[i];
			}
			Ok(C)
		}
	}

	pub fn vProduct() -> Vector {

	}
}
