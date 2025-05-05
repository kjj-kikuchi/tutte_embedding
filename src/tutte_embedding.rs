use crate::mesh::Mesh;
use nalgebra_sparse::{
    coo::CooMatrix,
    csr::CsrMatrix,
};

use std::{collections::BTreeSet, f64::consts::PI};

fn calc_vertices_digee(mesh: &Mesh) -> Vec<usize> {
    let mut digree: Vec<usize> = vec![0; mesh.vertices.len()];

    for he in &mesh.halfedges {
        digree[he.v_src(&mesh.faces)] += 1;
        if he.is_boundary() {
            digree[he.v_tgt(&mesh.faces)] += 1;
        }
    }

    digree
}

fn compute_max_length_boundary_vertices(mesh: &Mesh) -> Vec<usize> {
    // 境界ハーフエッジを保存するセット
    let mut boundary_he: BTreeSet<usize> = mesh
        .halfedges
        .iter()
        .enumerate()
        .filter(|(_, he)| he.is_boundary())
        .map(|(i, _)| i)
        .collect();

    let mut max_boundary_vertices = Vec::new();
    let mut max_boundary_len = 0.0;

    // boundary_he　が空になるまで
    while let Some(&he_start) = boundary_he.iter().next() {
        let mut boundary_v = Vec::new(); // 1つの境界サイクルの頂点集合
        let mut he_curr = he_start;
        let mut cycle_len = 0.0;

        loop {
            // 現在のハーフエッジの始点を追加
            let v_curr = mesh.halfedges[he_curr].v_src(&mesh.faces);
            boundary_v.push(v_curr);

            // 現在のハーフエッジの長さを加算
            cycle_len += mesh.halfedges[he_curr].length(mesh);

            // 現在のハーフエッジを削除
            boundary_he.remove(&he_curr);

            // 次の境界ハーフエッジを計算
            let opp = mesh.halfedges[he_curr].h_opp;
            let he_next = usize::try_from(-opp - 1).unwrap();

            // １周したら終了
            if he_next == he_start {
                break;
            }
            he_curr = he_next;
        }

        if cycle_len > max_boundary_len {
            max_boundary_len = cycle_len;
            max_boundary_vertices = boundary_v;
        }
    }
    max_boundary_vertices
}

fn construct_laplacian_triplets(
    mesh: &Mesh,
    boundary_vertices: &Vec<usize>,
    digree: &Vec<usize>,
) -> CooMatrix<f64> {
    let n = mesh.vertices.len();
    let mut triplets: CooMatrix<f64> = CooMatrix::new(n, n);
    for i in 0..n {
        triplets.push(i, i, 1.0);
    }

    for &bv in boundary_vertices {
        let he_out = &mesh.halfedges[mesh.h_out[bv]];
        triplets.push(
            he_out.v_src(&mesh.faces),
            he_out.v_tgt(&mesh.faces),
            -1.0 / (digree[bv] as f64),
        );
    }
    
    triplets
}

pub fn compute_nalgebra(
        mesh: &Mesh,
        x: &mut na::DVector<f64>,
        y: &mut na::DVector<f64>,
) {
    let digree = calc_vertices_digee(&mesh);
    let boundary_vertices = compute_max_length_boundary_vertices(&mesh);

    //ラプラシアン疎行列を作成
    let n = mesh.vertices.len();
    let triplets = construct_laplacian_triplets(&mesh, &boundary_vertices, &digree);    // COO
    let laplacian = CsrMatrix::from(&triplets);     // COO -> CSR

    // 右辺ベクトルを作成
    // let mut b: na::DMatrix<f64> = na::DMatrix::zeros(n, 2);
    let mut b_x = na::DVector::zeros(n);
    let mut b_y = na::DVector::zeros(n);
    for (i, &bv) in boundary_vertices.iter().enumerate() {
        let theta = 2.0 * PI * (i as f64) / (boundary_vertices.len() as f64);

        let (x, y) = (theta.cos(), theta.sin());
        // match (x, y) {
        //     (-1.0, _) => (x, 0.0),
        //     (_, -1.0) => (1.0, y),
        //     _         => (x,   y),
        // };
        b_x[bv] = x;
        b_y[bv] = y;
        // if b[(0, *bv)] == -1.0 {
        //     b[(1, *bv)] = 0.0;
        // }
        // if b[(1, *bv)] == -1.0 {
        //     b[(0, *bv)] = 1.0;
        // }
    }

    let tol = 1e-8;
    let max_iters = 1_000;
    *x = solve_sparse_cg(&laplacian, &b_x, tol, max_iters);
    *y = solve_sparse_cg(&laplacian, &b_y, tol, max_iters);
}