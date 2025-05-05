use std::env;
use std::process;
extern crate nalgebra as na;

pub mod file_io;
pub mod halfedge;
pub mod mesh;
pub mod tutte_embedding;

use mesh::Mesh;

fn main() {
    // 入力ファイル読み込み
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Number of command line argument is not one.");
        process::exit(1);
    }
    let filename = &args[1];

    println!("{}", filename);

    let mut mesh = Mesh::from_obj(filename);
    mesh.construct_halfedge_list();

    println!("Vertices : {}", mesh.vertices.len());
    println!("Faces : {}", mesh.faces.len());
    println!("Normals : {}", mesh.normal_vertex.len());

    println!("{}", mesh.vertices[0].transpose());
    println!("{}", mesh.faces[0].transpose());
}
