mod matrices;

pub use crate::matrices::*;

fn display(point: &[f32; 3]) {
    println!("x: {}", point[0]);
    println!("y: {}", point[1]);
    println!("z: {}", point[2]);
}

fn main() {
    const PI: f32 = 3.1416;
    /*
    let mut point = Vertex {
        coords: [1.0,2.0,3.0],
    };
    let mat = Matrix {
        columns: [[1.0,2.0,3.0],[1.0,2.0,4.0],[1.0,2.0,1.0]],
    };
    let mut mat2 = Matrix {
        columns: [[0.0,1.0,0.0],[1.0,0.0,0.0],[0.0,0.0,1.0]],
    };
    point.coords = matrices::Matrix::mtimes_v(&mat, &point.coords).unwrap();
    display(&point.coords);
    mat2 = matrices::Matrix::mtimes_m(&mat, &mat2).unwrap();
    mat2.display();
    */
    let mut mat = [[2.0,0.0,0.0],[0.0,0.0,1.0],[0.0,0.0,1.0]];
    let me = matrices::mtimes(&mat, &[1.0,2.0,3.0]).unwrap();
    println!("{}, {}, {}", me[0],me[1],me[2]);
    //mat.rotate_m(PI, 0.0, 0.0);
    //mat.display();
    //mat.translate_m(2.0, 5.0, 4.0);
    //mat.display();
}
