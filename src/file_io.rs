use std::fs::File;
use std::io::{BufRead, BufReader};

use super::Mesh;

impl Mesh {
    pub fn from_obj(filename: &String) -> Mesh {
        // ファイルを開く
        let file = File::open(filename).expect("file not found");
        let reader = BufReader::new(file);

        let mut mesh = Mesh::new();

        // 行ごとに処理
        for line in reader.lines() {
            let line = line.expect("Failed to read line.");
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let tokens: Vec<&str> = line.split_whitespace().collect();

            match tokens[0] {
                "v" => {
                    if let (Ok(x), Ok(y), Ok(z)) = (
                        tokens[1].parse::<f64>(),
                        tokens[2].parse::<f64>(),
                        tokens[3].parse::<f64>(),
                    ) {
                        mesh.vertices.push(na::Vector3::new(x, y, z));
                    }
                }
                "vn" => {
                    if let (Ok(x), Ok(y), Ok(z)) = (
                        tokens[1].parse::<f64>(),
                        tokens[2].parse::<f64>(),
                        tokens[3].parse::<f64>(),
                    ) {
                        mesh.normal_vertex.push(na::Vector3::new(x, y, z));
                    }
                }
                "f" => {
                    let mut indices = Vec::new();

                    for i in 1..=3 {
                        let parts: Vec<&str> = tokens[i].split("//").collect();
                        if let Ok(idx) = parts[0].parse::<usize>() {
                            indices.push(idx-1);
                        }
                    }
                    mesh.faces.push(na::Vector3::new(indices[0], indices[1], indices[2]));
                }
                _ => {} // do nothing
            }
        }
        mesh
    }
}