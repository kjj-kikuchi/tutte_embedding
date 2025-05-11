use std::env;
use std::process;
extern crate nalgebra as na;

pub mod file_io;
pub mod halfedge;
pub mod mesh;
pub mod tutte_embedding;

use file_io::write_obj;
use mesh::Mesh;
use tutte_embedding::compute_rsparse;

fn main() {
    // 入力ファイル読み込み
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Number of command line argument is not one.");
        process::exit(1);
    }
    let filename = &args[1];

    // メッシュ構築
    let mut mesh = Mesh::from_obj(filename);
    mesh.construct_halfedge_list();

    // tutte埋め込み計算
    compute_rsparse(&mut mesh);

    // 出力
    write_obj(&filename, &mesh).unwrap();
}
