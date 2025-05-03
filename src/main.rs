use std::env;
use std::process;

mod mesh;
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

    let mesh = Mesh::from_obj(filename);

    println!("Vertices : {}", mesh.vertices.len());
    println!("Faces : {}", mesh.faces.len());
    println!("Normals : {}", mesh.normal_vertex.len());

    println!("{}", mesh.vertices[0].transpose());
    println!("{}", mesh.faces[0].transpose());
}
