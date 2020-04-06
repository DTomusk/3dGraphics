pub struct Vertex {
	//leaving: &'a Edge,
	pub coords: Coords,
}

pub struct Coords {
	pub x: i32,
	pub y: i32,
	pub z: i32,
}

impl Coords {
	// functions to transform coordinates
	pub fn display(self: &Self) {
		println!("x: {}", self.x);
		println!("y: {}", self.y);
		println!("z: {}", self.z);
	}

	pub fn translate(&mut self, dx: i32, dy: i32, dz: i32) {
		self.x += dx;
		self.y += dy;
		self.z += dz;
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

pub struct Matrix<'a> {
	columns: Vec<Coords>,
}


*/
