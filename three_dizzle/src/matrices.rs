pub struct Vertex {
	//leaving: &'a Edge,
	pub coords: [i32; 3],
}

pub struct Matrix {
	pub columns: Vec<[i32; 3]>,
}

impl Matrix {
	// will return either a matrix or a coords
	pub fn mtimesv(a: Matrix, b: [i32; 3]) -> Result<[i32; 3], &'static str> {
		if a.columns.len() != 3 {
			Err("Matrix must have three columns to multiply with vector")
		} else {
			let mut res = [0,0,0];
			for i in 0..2 {
				for col in &a.columns {
					for j in 0..2 {
						res[i] += col[i]*b[j];
					};
				};
			};
			Ok(res)
		}
	}

	//pub fn mtimesm(A: Matrix, B: Matrix) -> Result<Matrix, ()> {

	//}
}

/*
pub struct Edge<'a> {
	twin: &'a Edge,
	next: &'a Edge,
	origin: &'a Vertex,
	incident: &'a Face,
}

pub struct Face<'a> {
	incident: &'a Edge,
}




*/
