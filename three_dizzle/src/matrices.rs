struct Vertex {
	leaving: Option<usize>,
	coords: [f32; 3],
}

pub struct Edge {
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
	pub edges: Vec<Edge>,
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

	// this can be a slow process
	// but once it's initialised it's easy to work with
	pub fn make_poly(edge_list: &Vec<[[f32;3];2]>) -> Poly {
		// add all vertices
		let mut poly = Poly::empty_poly();
		for edge in edge_list {
			let mut start_pointer: Option<usize> = None;
			let mut end_pointer: Option<usize> = None;
			match Poly::check_vert(&edge[0], &poly) {
				Some(i) => start_pointer = Some(i),
				None => start_pointer = Some(poly.add_vertex(edge[0])),
			};
			// this is an exact repeat of what's above but I'm not sure it's worth it to
			// make it a function of its own
			match Poly::check_vert(&edge[1], &poly) {
				Some(i) => end_pointer = Some(i),
				None => end_pointer = Some(poly.add_vertex(edge[1])),
			};
			let first_pointer = poly.add_half(start_pointer.unwrap());
			let second_pointer = poly.add_half(end_pointer.unwrap());

			// twins are also wrong with the reordering
			poly.edges[first_pointer].twin = Some(second_pointer);
			poly.edges[second_pointer].twin = Some(first_pointer);

			// this part doesn't apply anymore
			if poly.verts[start_pointer.unwrap()].leaving == None {
				poly.verts[start_pointer.unwrap()].leaving = Some(first_pointer);
			};
			if poly.verts[end_pointer.unwrap()].leaving == None {
				poly.verts[end_pointer.unwrap()].leaving = Some(second_pointer);
			};
		};
		//poly.edges.sort_by(|b, a| b.origin.cmp(&a.origin))
		// the question is how often do I plan on reordering elements?
		// pointers need to be updated very carefully
		// we can set leaving pointers after the fact (do we even need them?)

		poly
	}

	pub fn quicksort(&mut self, lower: usize, upper: usize) {
	    let mut pivot: usize = upper/2;
	    // want to iterate counters either side of the pivot
	    let mut first_counter = lower;
	    let mut second_counter = pivot + 1;
		println!("Pivot: {}", pivot);
		println!("First: {}", first_counter);
		println!("Second: {}", second_counter);
	    while (first_counter < pivot) | (second_counter < upper) {
	        // if the current things need to be swapped then swap them
	        if (first_counter < pivot) & (second_counter < upper) {
	            if (self.edges[first_counter].origin > self.edges[pivot].origin) & (self.edges[second_counter].origin < self.edges[pivot].origin) {
	                self.swap(first_counter, second_counter);
	                first_counter+=1;
	                second_counter+=1;
	            } else if self.edges[first_counter].origin > self.edges[pivot].origin {
	                second_counter+=1;
	            } else if self.edges[second_counter].origin < self.edges[pivot].origin {
	                first_counter+=1;
	            } else {
	                first_counter += 1;
	                second_counter += 1;
	            };
	        } else if first_counter < pivot {
	            if (self.edges[first_counter].origin > self.edges[pivot].origin) & (first_counter == pivot - 1) {
	                self.swap(pivot, first_counter);
	                pivot -= 1;
	            } else if self.edges[first_counter].origin > self.edges[pivot].origin {
	                self.swap(pivot - 1, pivot);
	                self.swap(pivot, first_counter);
	                pivot -= 1;
	            } else {
	                first_counter += 1;
	            };
	        } else if second_counter < upper {
	            if (self.edges[second_counter].origin < self.edges[pivot].origin) & (second_counter == pivot + 1) {
	                self.swap(pivot, second_counter);
	                pivot += 1;
	            } else if self.edges[second_counter].origin < self.edges[pivot].origin {
	                self.swap(pivot + 1, pivot);
	                self.swap(pivot, second_counter);
	                pivot += 1;
	            // if one thing needs to be moved hold on to it and wait for the other
	            } else {
	                second_counter += 1;
	            };
	        };
	    };
		println!("Upper: {}", upper);
		println!("Lower: {}", lower);
	    if upper >= (2 + lower) {
			println!("Sorting lower end");
	        self.quicksort(lower, pivot);
			println!("Sorting upper end");
	        self.quicksort(pivot, upper);
	    };
	}

	pub fn swap(&mut self, first: usize, second: usize) {
		// this swaps them, but it doesn't set the pointers right
		let i: usize = self.edges[first].twin.unwrap();
		let j: usize = self.edges[second].twin.unwrap();
		let swap_twin = self.edges[i].twin;
		self.edges[i].twin = self.edges[j].twin;
		self.edges[j].twin = swap_twin;

		if self.verts[self.edges[first].origin].leaving.unwrap() == first {
			self.verts[self.edges[first].origin].leaving = Some(second);
		};

		if self.verts[self.edges[second].origin].leaving.unwrap() == second {
			self.verts[self.edges[second].origin].leaving = Some(first);
		};

		let temp_o = self.edges[first].origin;
		self.edges[first].origin = self.edges[second].origin;
		self.edges[second].origin = temp_o;

		let temp_twin = self.edges[first].twin;
		self.edges[first].twin = self.edges[second].twin;
		self.edges[second].twin = temp_twin;
	}

	pub fn check_vert(point: &[f32;3], poly: &Poly) -> Option<usize> {
		let mut index = None;
		for (i, v) in poly.verts.iter().enumerate() {
			if (v.coords[0] == point[0]) & (v.coords[1] == point[1]) & (v.coords[2] == point[2]) {
				index = Some(i);
			};
		};
		index
	}

	pub fn add_vertex(&mut self, coords: [f32;3]) -> usize {
		self.verts.push(Vertex {
			leaving: None,
			coords: coords,
		});
		self.verts.len() -1
	}

	fn add_half(&mut self, origin: usize) -> usize {
		self.edges.push(Edge{
			twin: None,
			next: None,
			origin: origin,
		});
		self.edges.len() -1
	}

	pub fn create_vertex(coords: [f32;3]) -> Poly {
		let mut poly = Poly::empty_poly();
		poly.add_vertex(coords);
		poly
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

	pub fn create_path(path: &Vec<[f32;3]>) -> Poly {
		let mut poly = Poly::create_line(path[0], path[1]);
		let mut latest_vert = 1;
		let mut edge1 = 0;
		let mut edge2 = 1;
		for i in 2..path.len(){
			latest_vert = poly.add_vertex(path[i]);
			edge1 = poly.add_half(latest_vert-1);
			edge2 = poly.add_half(latest_vert);
			poly.verts[latest_vert].leaving = Some(edge2);
			poly.edges[edge1].twin = Some(edge2);
			poly.edges[edge2].twin = Some(edge1);
			poly.edges[edge2].next = Some(edge2-2);
			poly.edges[edge1-2].next =  Some(edge1);
		};
		poly
	}

	pub fn create_cycle(cycle: &Vec<[f32;3]>) -> Poly {
		let mut poly = Poly::create_path(&cycle);
		let pen = poly.add_half(0);
		let last = poly.add_half(poly.verts.len()-1);
		poly.edges[pen].twin = Some(last);
		poly.edges[last].twin = Some(pen);
		poly.edges[pen].next = Some(pen-1);
		poly.edges[last].next = Some(0);
		poly.edges[last-3].next = Some(last);
		poly.edges[1].next = Some(pen);
		poly
	}

	pub fn display(&self) {
		println!("Vertices:");
		for v in &self.verts {
			print!("x: {}, y: {}, z: {}, ", v.coords[0], v.coords[1], v.coords[2]);
			match v.leaving {
				Some(i) => println!("leaving: {}", i),
				None => println!("leaving: none"),
			};
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

	pub fn check_cycle(&self) {
		let mut ret = false;
		let start = 0;
		let mut index = 0;
		while ret == false {
			if self.edges[index].next == None {
				println!("Not cyclic");
				break;
			} else if self.edges[index].next.unwrap() == start {
				print!("Cyclic");
				ret = true;
			} else {
				index = self.edges[index].next.unwrap();
			};
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
