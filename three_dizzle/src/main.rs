mod matrices;

pub use crate::matrices::*;

fn display(point: &[i32; 3]) {
    println!("x: {}", point[0]);
    println!("y: {}", point[1]);
    println!("z: {}", point[2]);
}

fn main() {
    println!("Hello, world!");
    let mut point = Vertex {
        coords: [1,2,3],
    };
    let mut mat = Matrix {
        columns: [[1,2,3],[1,2,4],[1,2,1]],
    };
    let mut mat2 = Matrix {
        columns: [[0,1,0],[1,0,0],[0,0,1]],
    };
    point.coords = matrices::Matrix::mtimesv(&mat, &point.coords).unwrap();
    display(&point.coords);
    mat2 = matrices::Matrix::mtimesm(&mat, &mat2).unwrap();
    mat2.display();
}
