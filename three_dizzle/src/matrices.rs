pub struct Vertex {
	//leaving: &'a Edge,
	pub coords: [i32; 3],
}

pub struct Matrix {
	pub columns: [[i32; 3]; 3],
}

impl Matrix {
	// will return either a matrix or a coords
	pub fn mtimesv(a: &Matrix, b: &[i32; 3]) -> Result<[i32; 3], &'static str> {
		if a.columns.len() != 3 {
			return Err("Matrix must have three columns to multiply with vector")
		} else {
			let mut res = [0,0,0];
			for i in 0..3 {
				for j in 0..3 {
					res[i] += a.columns[j][i]*b[j];
				};
			};
			return Ok(res)
		};
	}

	pub fn mtimesm(a: &Matrix, b: &Matrix) -> Result<Matrix, ()> {
		// assume can multiply
		let mut res = [[0;3];3];
		for i in 0..3 {
			res[i] = Matrix::mtimesv(&a, &b.columns[i]).unwrap();
		}
		return Ok(Matrix{columns: res})
	}

	pub fn display(&self) {
		for i in 0..3 {
			for j in 0..3 {
				println!("{}", self.columns[i][j]);
			}
		}
	}
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
