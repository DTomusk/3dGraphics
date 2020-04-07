pub struct Vertex {
	//leaving: &'a Edge,
	pub coords: [f32; 3],
}

pub fn mtimes(a: &Vec<[f32;3]>, b: &[f32;3]) -> Result<[f32;3], &'static str> {
	if a[0].len() != 3 {
		return Err("Matrix must have three columns to multiply with vector")
	} else {
		let mut res = [0.0;3];
		for i in 0..3 {
			for j in 0..3 {
				res[i] += a[j][i]*b[j];
			};
		};
		return Ok(res)
	};
}

/*
pub fn mtimes_m(a: &Matrix, b: &Matrix) -> Result<Matrix, ()> {
	// assume can multiply
	let mut res = [[0.0;3];3];
	for i in 0..3 {
		res[i] = Matrix::mtimes_v(&a, &b.columns[i]).unwrap();
	}
	Ok(Matrix{columns: res})
}

pub fn rotate_v(a: [f32;3], x: f32, y: f32, z: f32) -> Result<[f32; 3], &'static str> {
	// hard coded rotation matrices
	let xrot = Matrix{columns: [[1.0,0.0,0.0],[0.0,x.cos(),x.sin()],[0.0,-x.sin(),x.cos()]]};
	let yrot = Matrix{columns: [[y.cos(),0.0,-y.sin()],[0.0,1.0,0.0],[y.sin(),0.0,y.cos()]]};
	let zrot = Matrix{columns: [[z.cos(),z.sin(),0.0],[-z.sin(),z.cos(),0.0],[0.0,0.0,1.0]]};
	let rot = Matrix::mtimes_m(&zrot, &Matrix::mtimes_m(&yrot, &xrot).unwrap()).unwrap();
	Ok(Matrix::mtimes_v(&rot, &a).unwrap())
}

pub fn rotate_m(&mut self, x: f32, y: f32, z: f32) {
	for i in 0..3 {
		self.columns[i] = Matrix::rotate_v(self.columns[i], x, y, z).unwrap();
	};
}

pub fn translate_v(a: [f32;3], x: f32, y: f32, z: f32) -> Result<[f32; 3], &'static str> {
	let mut res = [x, y, z];
	res[0] += a[0];
	res[1] += a[1];
	res[2] += a[2];
	return Ok(res)
}

pub fn translate_m(&mut self, x: f32, y: f32, z: f32) {
	for i in 0..3 {
		self.columns[i] = Matrix::translate_v(self.columns[i], x, y, z).unwrap();
	};
}

pub fn scale_v(a: [f32;3], s: f32) -> Result<[f32; 3], &'static str> {
	let mut res = a;
	for i in 0..3 {
		res[i] *= s;
	}
	return Ok(res)
}

pub fn scale_m(&mut self, s: f32) {
	for i in 0..3 {
		self.columns[i] = Matrix::scale_v(self.columns[i], s).unwrap();
	};
}
*/

/*
pub fn display(a: &[[f32]]) {
	for i in 0..3 {
		println!("{} {} {}", a[0][i] as i32, a[1][i] as i32, a[2][i] as i32);
	}
}
*/

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
