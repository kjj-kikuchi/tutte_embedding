use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

use super::Mesh;

impl Mesh {
    pub fn from_obj(filename: &str) -> Mesh {
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

pub fn write_obj(
    filename: &str,
    mesh: &Mesh
) -> std::io::Result<()> {
    let base = Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(filename);

    let ofilename = format!("{}_tutte_embedding.obj", base);
    let file = File::create(&ofilename)?;
    let mut w = BufWriter::new(file);

    for v in &mesh.vertices {
        writeln!(w, "v {} {} {}", v.x, v.y, v.z)?;
    }

    for vn in &mesh.normal_vertex {
        writeln!(w, "vn {} {} {}", vn.x, vn.y, vn.z)?;
    }

    for f in &mesh.faces {
        let i0 = f[0] + 1;
        let i1 = f[1] + 1;
        let i2 = f[2] + 1;
        writeln!(w, "f {}//{} {}//{} {}//{}", i0, i0, i1, i1, i2, i2)?;
    }

    w.flush()?;
    Ok(())
}