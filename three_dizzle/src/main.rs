mod matrices;

//use crate::matrices::*;
use std::rc::Rc;

fn main() {
    let mat = vec![[2.0,0.0,0.0],[0.0,0.0,1.0],[0.0,0.0,1.0]];
    let _me = matrices::mtimes(&mat, &[1.0,2.0,3.0]).unwrap();

    let mut point1 = matrices::Vertex {
        leaving: -1,
        coords: [1.0, 0.0, 0.0],
    };
    let mut point2 = matrices::Vertex {
        leaving: -1,
        coords: [0.0, 1.0, 0.0],
    };
    let mut half1 = matrices::Edge {
        twin: -1,
        next: -1,
        origin: -1,
    };
    let mut half2 = matrices::Edge {
        twin: -1,
        next: -1,
        origin: -1,
    };

    let mut vlist: std::vec::Vec<matrices::Vertex> = vec![];
    let mut elist: std::vec::Vec<matrices::Edge> = vec![];
    vlist.push(point1);
    elist.push(half1);
    vlist[0].leaving=0;
    elist[0].origin=0;
    vlist.push(point2);
    elist.push(half2);
    vlist[1].leaving=1;
    elist[1].origin=1;
    elist[0].twin=1;
    elist[1].twin=0;
    println!("{}", elist[0].twin);

    // a shape will consist of a vector of faces, a vector of half edges, and a vector of vertices
    // pointers will be indices in each other's arrays 
}
