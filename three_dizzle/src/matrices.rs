struct Vertex {
	leaving: Option<usize>,
	coords: [f32; 3],
}

struct Edge {
	twin: Option<usize>,
	next: Option<usize>,
	origin: usize,
	//incident: &Face,
}

struct Face {
	incident: Option<usize>,
}

pub struct Poly {
	verts: Vec<Vertex>,
	edges: Vec<Edge>,
	faces: Vec<Face>,
}

impl Poly {
	pub fn empty_poly() -> Poly {
		let mut poly = Poly {
			verts: vec![],
			edges: vec![],
			faces: vec![]
		};
		poly
	}

	pub fn add_vertex(&mut self, coords: [f32;3]) -> usize {
		self.verts.push(Vertex {
			leaving: None,
			coords: coords,
		});
		self.verts.len() -1
	}

	pub fn create_vertex(&mut self, coords: [f32;3]) {
		self.add_vertex(coords);
	}

	fn add_half(&mut self, origin: usize) -> usize {
		self.edges.push(Edge{
			twin: None,
			next: None,
			origin: origin,
		});
		self.edges.len() -1
	}

	pub fn create_line(start: [f32;3], end: [f32;3]) -> Poly {
		let mut poly = Poly::empty_poly();
		let start_index = poly.add_vertex(start);
		let end_index = poly.add_vertex(end);
		let half_pointer_1 = poly.add_half(start_index);
		let half_pointer_2 = poly.add_half(end_index);
		if poly.verts[start_index].leaving == None {
			poly.verts[start_index].leaving = Some(half_pointer_1);
		};
		if poly.verts[end_index].leaving == None {
			poly.verts[end_index].leaving = Some(half_pointer_2);
		};
		poly.edges[half_pointer_1].twin = Some(half_pointer_2);
		poly.edges[half_pointer_2].twin = Some(half_pointer_1);
		poly
	}

	pub fn display(&self) {
		println!("Vertices:");
		for v in &self.verts {
			println!("x: {}, y: {}, z: {}", v.coords[0], v.coords[1], v.coords[2]);
		};
		println!("");
		println!("Edges:");
		for e in &self.edges {
			println!("Origin: {}", e.origin);
			match e.next {
				Some(n) => println!("Next: {}", n),
				None => println!("No next"),
			};
			match e.twin {
				Some(t) => println!("Twin: {}", t),
				None => println!("No twin"),
			};
			println!("");
		};
	}

/*
	fn add_face() {

	}
*/
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
