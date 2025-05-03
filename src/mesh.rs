use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Mesh {
    pub vertices: Vec<nalgebra::Vector3<f64>>,
    pub faces: Vec<nalgebra::Vector3<i32>>,
    pub normal_vertex: Vec<nalgebra::Vector3<f64>>,
}

impl Mesh {
    pub fn new() -> Mesh {
        let vertices = Vec::new();
        let faces = Vec::new();
        let normal_vertex = Vec::new();
        Mesh {
            vertices,
            faces,
            normal_vertex,
        }
    }

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
                        mesh.vertices.push(nalgebra::Vector3::new(x, y, z));
                    }
                }
                "vn" => {
                    if let (Ok(x), Ok(y), Ok(z)) = (
                        tokens[1].parse::<f64>(),
                        tokens[2].parse::<f64>(),
                        tokens[3].parse::<f64>(),
                    ) {
                        mesh.normal_vertex.push(nalgebra::Vector3::new(x, y, z));
                    }
                }
                "f" => {
                    let mut indices = Vec::new();

                    for i in 1..=3 {
                        let parts: Vec<&str> = tokens[i].split("//").collect();
                        if let Ok(idx) = parts[0].parse::<i32>() {
                            indices.push(idx-1);
                        }
                    }
                    mesh.faces.push(nalgebra::Vector3::new(indices[0], indices[1], indices[2]));
                }
                _ => {} // do nothing
            }
        }
        mesh
    }
}
