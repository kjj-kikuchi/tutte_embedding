use crate::mesh::Mesh;
use std::{collections::BTreeSet, f64::consts::PI};
use rsparse::{data::{Sprs, Trpl}, lusol};
// use nalgebra_sparse::{factorization::CscCholesky, CooMatrix, CscMatrix};

pub fn calc_vertices_degree(mesh: &Mesh) -> Vec<usize> {
    let mut degree: Vec<usize> = vec![0; mesh.vertices.len()];

    for h in &mesh.halfedges {
        degree[h.v_src(&mesh.faces)] += 1;
        if h.is_boundary() {
            degree[h.v_tgt(&mesh.faces)] += 1;
        }
    }

    degree
}

pub fn compute_longest_boundary_cycle(
    mesh: &Mesh,
    boundary_vertices_set: &mut BTreeSet<usize>,
    boundary_vertices: &mut Vec<usize>
) {
    // 境界ハーフエッジを保存するセット
    let mut boundary_h: BTreeSet<usize> = mesh
        .halfedges
        .iter()
        .enumerate()
        .filter(|(_, h)| h.is_boundary())
        .map(|(i, _)| i)
        .collect();

    let mut max_boundary_len = 0.0;

    // boundary_h　が空になるまで
    while let Some(&h_start) = boundary_h.iter().next() {
        let mut bv_set = BTreeSet::new();
        let mut boundary_v = Vec::new();      // 1つの境界サイクルの頂点集合
        let mut h_curr = h_start;
        let mut boundary_len = 0.0;

        loop {
            let v_curr = mesh.halfedges[h_curr].v_src(&mesh.faces);
            bv_set.insert(v_curr);
            boundary_v.push(v_curr);
            // boundary_cycle_h.insert(h_curr);

            boundary_len += mesh.halfedges[h_curr].norm(mesh);

            boundary_h.remove(&h_curr);

            let h_curr_opp = mesh.halfedges[h_curr].h_opp;
            let h_next = usize::try_from(-h_curr_opp - 1).unwrap();

            if h_next == h_start {
                break;
            }
            h_curr = h_next;
        }

        if boundary_len > max_boundary_len {
            max_boundary_len = boundary_len;
            *boundary_vertices_set = bv_set;
            *boundary_vertices = boundary_v;
        }
    }
}

// fn make_symmetric_laplacian_coo(
//     mesh: &Mesh,
//     boundary_vertices_set: &BTreeSet<usize>,
//     degree: &Vec<usize>
// ) -> CooMatrix<f64> {
//     let n = mesh.vertices.len();
//     let mut triplets: CooMatrix<f64> = CooMatrix::new(n, n);
//     for i in 0..n {
//         triplets.push(i, i, 1.0);
//     }

//     for h in &mesh.halfedges {
//         // どちらも内部頂点となる要素 (v_src, v_tgt) に値 -1.0 / d(v_src) を設定
//         if !boundary_vertices_set.contains(&h.v_src(&mesh.faces)) &&
//            !boundary_vertices_set.contains(&h.v_tgt(&mesh.faces)) {
//             triplets.push(
//                 h.v_src(&mesh.faces),
//                 h.v_tgt(&mesh.faces),
//                 -1.0 / (degree[h.v_src(&mesh.faces)] as f64),
//             );
//         }
//     }

//     triplets
// }

// fn make_rhs_vector_of_symmetric_laplacian(
//     mesh: &Mesh,
//     boundary_vertices_set: &BTreeSet<usize>,
//     boundary_vertices: &Vec<usize>,
//     degree: &Vec<usize>
// ) -> (na::DVector<f64>, na::DVector<f64>) {
//     let n = mesh.vertices.len();
//     let mut bx = na::DVector::zeros(n);
//     let mut by = na::DVector::zeros(n);

//     // 境界頂点を円周上に配置する
//     for (i, &b_v) in boundary_vertices.iter().enumerate() {
//         let theta = 2.0 * PI * (i as f64) / (boundary_vertices.len() as f64);
//         bx[b_v] = theta.cos();
//         by[b_v] = theta.sin();
//     }

//     // コレスキー分解ができるよう，ラプラシアン行列を対称行列するために，
//     // 行列の要素を右辺ベクトルに移項し，足し合わせる
//     for h in &mesh.halfedges {
//         // 内部頂点 i と境界頂点 j の要素 (i, j) の値 -1.0 / d(i)　を bi に足す
//         if !boundary_vertices_set.contains(&h.v_src(&mesh.faces)) &&
//             boundary_vertices_set.contains(&h.v_tgt(&mesh.faces)) {
//             let val = 1.0 / (degree[h.v_src(&mesh.faces)] as f64);
//             bx[h.v_src(&mesh.faces)] += bx[h.v_tgt(&mesh.faces)] * val;
//             by[h.v_src(&mesh.faces)] += by[h.v_tgt(&mesh.faces)] * val;
//         }
//     }
    
//     (bx, by)
// }

// pub fn compute_nalgebra(
//     mesh: &Mesh,
//     x: &mut na::DVector<f64>,
//     y: &mut na::DVector<f64>
// ) {
//     // 頂点次数を計算
//     println!("1...");
//     let degree = calc_vertices_degree(&mesh);

//     // 周長が最長となる境界サイクルを計算
//     println!("2...");
//     let mut boundary_vertices_set = BTreeSet::new();
//     let mut boundary_vertices = Vec::new();
//     compute_longest_boundary_cycle(&mesh, &mut boundary_vertices_set, &mut boundary_vertices);

//     //ラプラシアン疎行列を作成
//     println!("3...");
//     let triplets = make_symmetric_laplacian_coo(&mesh, &boundary_vertices_set, &degree); // COO
//     let laplacian = CscMatrix::from(&triplets); // COO -> CSC

//     // 右辺ベクトルを作成
//     println!("4...");
//     let (bx, by) = make_rhs_vector_of_symmetric_laplacian(&mesh, &boundary_vertices_set, &boundary_vertices, &degree);

//     println!("5...");
//     let factor = CscCholesky::factor(&laplacian).unwrap();

//     // let x_result = factor.solve(&bx);
//     // let y_result = factor.solve(&by);
// }

// =======================================================================================================================

fn make_laplacian_triplets(
    mesh: &Mesh,
    boundary_vertices_set: &BTreeSet<usize>,
    degree: &Vec<usize>
) -> Trpl<f64> {
    let n = mesh.vertices.len();
    let mut triplets: Trpl<f64> = Trpl::new();
    for i in 0..n {
        triplets.append(i, i, 1.0);
    }

    for h in &mesh.halfedges {
        // 内部頂点 v_src に対して， 要素 (v_src, v_tgt) に値 -1.0 / d(v_src)　を設定
        if !boundary_vertices_set.contains(&h.v_src(&mesh.faces)) {
            triplets.append(
                h.v_src(&mesh.faces),
                h.v_tgt(&mesh.faces),
                -1.0 / (degree[h.v_src(&mesh.faces)] as f64),
            );
        }
    }

    triplets
}

fn make_rhs_vector(
    mesh: &Mesh,
    boundary_vertices: &Vec<usize>
) -> (Vec<f64>, Vec<f64>) {
    let n = mesh.vertices.len();
    let mut bx = vec![0.0; n];
    let mut by = vec![0.0; n];

    // 境界頂点を円周上に配置する
    for (i, &b_v) in boundary_vertices.iter().enumerate() {
        let theta = 2.0 * PI * (i as f64) / (boundary_vertices.len() as f64);
        bx[b_v] = theta.cos();
        by[b_v] = theta.sin();
    }
    
    (bx, by)
}

fn make_parametrized_mesh(
    new_x: Vec<f64>,
    new_y: Vec<f64>,
    new_mesh: &mut Mesh
) {
    for (i, v) in new_mesh.vertices.iter_mut().enumerate() {
        v.x = new_x[i];
        v.y = new_y[i];
        v.z = 0.0;
    }
    // new_mesh.calc_face_normals();
}

pub fn compute_rsparse(
    mesh: &mut Mesh
) {
    // 頂点次数を計算
    println!("1...");
    let degree = calc_vertices_degree(&mesh);

    // 周長が最長となる境界サイクルを計算
    println!("2...");
    let mut boundary_vertices_set = BTreeSet::new();
    let mut boundary_vertices = Vec::new();
    compute_longest_boundary_cycle(&mesh, &mut boundary_vertices_set, &mut boundary_vertices);

    //ラプラシアン疎行列を作成
    println!("3...");
    let triplets = make_laplacian_triplets(&mesh, &boundary_vertices_set, &degree); // COO
    let laplacian = Sprs::new_from_trpl(&triplets);    // Triplet -> CSC

    // 右辺ベクトルを作成
    println!("4...");
    let (mut bx, mut by) = make_rhs_vector(&mesh, &boundary_vertices);

    // 線形システムを計算
    println!("5...");
    lusol(&laplacian, &mut bx, 1, 1e-6).unwrap();
    lusol(&laplacian, &mut by, 1, 1e-6).unwrap();

    // パラメータ化されたメッシュに変換
    println!("6...");
    make_parametrized_mesh(bx, by, mesh);
}