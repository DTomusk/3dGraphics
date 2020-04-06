mod matrices;

pub use crate::matrices::*;

fn main() {
    println!("Hello, world!");
    let mut point = Vertex {
        coords: Coords {x:1,y:2,z:3},
    };
    point.coords.display();
    point.coords.translate(1,2,3);
    point.coords.display();
}
