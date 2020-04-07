mod matrices;

pub use crate::matrices::*;

fn main() {
    let mat = vec![[2.0,0.0,0.0],[0.0,0.0,1.0],[0.0,0.0,1.0]];
    let _me = matrices::mtimes(&mat, &[1.0,2.0,3.0]).unwrap();
}
