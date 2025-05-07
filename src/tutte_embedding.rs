use crate::mesh::Mesh;
use nalgebra_sparse::{
    coo::CooMatrix,
    csr::CsrMatrix,
};

use std::{collections::BTreeSet, f64::consts::PI};

fn calc_vertices_digree(mesh: &Mesh) -> Vec<usize> {
    let mut digree: Vec<usize> = vec![0; mesh.vertices.len()];

    for he in &mesh.halfedges {
        digree[he.v_src(&mesh.faces)] += 1;
        if he.is_boundary() {
            digree[he.v_tgt(&mesh.faces)] += 1;
        }
    }

    digree
}

// fn compute_longest_boundary_cycle_vertices(mesh: &Mesh) -> BTreeSet<usize> {
fn compute_longest_boundary_cycle_halfedges(mesh: &Mesh) -> BTreeSet<usize> {
    // 境界ハーフエッジを保存するセット
    let mut boundary_he: BTreeSet<usize> = mesh
        .halfedges
        .iter()
        .enumerate()
        .filter(|(_, he)| he.is_boundary())
        .map(|(i, _)| i)
        .collect();

    // let mut max_boundary_vertices = BTreeSet::new();
    let mut longest_boundary_cycle_he = BTreeSet::new();
    let mut max_boundary_len = 0.0;

    // boundary_he　が空になるまで
    while let Some(&he_start) = boundary_he.iter().next() {
        let mut boundary_cycle_he = BTreeSet::new(); // 1つの境界サイクルのハーフエッジ集合
        let mut he_curr = he_start;
        let mut boundary_len = 0.0;

        loop {
            boundary_cycle_he.insert(he_curr);
            boundary_len += mesh.halfedges[he_curr].norm(mesh);
            boundary_he.remove(&he_curr);

            let opp = mesh.halfedges[he_curr].h_opp;
            let he_next = usize::try_from(- opp - 1).unwrap();

            if he_next == he_start {
                break;
            }
            he_curr = he_next;
        }

        if boundary_len > max_boundary_len {
            max_boundary_len = boundary_len;
            longest_boundary_cycle_he = boundary_cycle_he;
        }
    }
    longest_boundary_cycle_he
}

fn construct_laplacian_coo(
    mesh: &Mesh,
    boundary_halfedges: &BTreeSet<usize>,
    digree: &Vec<usize>,
) -> CooMatrix<f64> {
    let n = mesh.vertices.len();
    let mut triplets: CooMatrix<f64> = CooMatrix::new(n, n);
    for i in 0..n {
        triplets.push(i, i, 1.0);
    }

    for (i, he) in mesh.halfedges.iter().enumerate() {
        if !boundary_halfedges.contains(&i) {
            triplets.push(
                he.v_src(&mesh.faces),
                he.v_tgt(&mesh.faces),
                -1.0 / (digree[he.v_src(&mesh.faces)] as f64),
            );
        }
    }
    
    triplets
}

pub fn compute_nalgebra(
        mesh: &Mesh,
        x: &mut na::DVector<f64>,
        y: &mut na::DVector<f64>,
) {
    let digree = calc_vertices_digree(&mesh);
    let boundary_halfedges = compute_longest_boundary_cycle_halfedges(&mesh);

    //ラプラシアン疎行列を作成
    let n = mesh.vertices.len();
    let triplets = construct_laplacian_coo(&mesh, &boundary_halfedges, &digree);    // COO
    let laplacian = CsrMatrix::from(&triplets);     // COO -> CSR

    // 右辺ベクトルを作成
    // let mut b: na::DMatrix<f64> = na::DMatrix::zeros(n, 2);
    let mut b_x = na::DVector::zeros(n);
    let mut b_y = na::DVector::zeros(n);
    for (i, &b_he) in boundary_halfedges.iter().enumerate() {
        let theta = 2.0 * PI * (i as f64) / (boundary_halfedges.len() as f64);
        let (x, y) = (theta.cos(), theta.sin());
        let bv = mesh.halfedges[b_he].v_src(&mesh.faces);
        b_x[bv] = x;
        b_y[bv] = y;
    }

}