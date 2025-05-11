use crate::halfedge::Halfedge;
use std::cmp::{max, min};
// use std::collections::BTreeMap;

pub struct Mesh {
    pub vertices: Vec<na::Vector3<f64>>,
    pub faces: Vec<na::Vector3<usize>>,
    pub normal_vertex: Vec<na::Vector3<f64>>,
    pub normal_face: Vec<na::Vector3<f64>>,
    pub halfedges: Vec<Halfedge>,
    pub h_out: Vec<usize>
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertices: Vec::new(),
            faces: Vec::new(),
            normal_vertex: Vec::new(),
            normal_face: Vec::new(),
            halfedges: Vec::new(),
            h_out: Vec::new(),
        }
    }

    pub fn construct_halfedge_list(&mut self) {
        // h_oppを計算(二分木)
        // let mut h_map = BTreeMap::new();

        // for i in 0..self.faces.len() {
        //     for j in 0..3 {
        //         let mut h = Halfedge::new(3 * i + j);

        //         let key      = (h.v_src(&self.faces), h.v_tgt(&self.faces));
        //         let key_swap = (h.v_tgt(&self.faces), h.v_src(&self.faces));

        //         match h_map.get(&key_swap) {
        //             Some(&opp_idx) => {
        //                 h.h_opp = opp_idx as isize;
        //                 self.halfedges[opp_idx as usize].h_opp = (3 * i + j) as isize;
        //             }
        //             None => {
        //                 h_map.insert(key, 3 * i + j);
        //             }
        //         }

        //         self.halfedges.push(h);
        //     }
        // }

        // h_oppを計算(ソート)
        let mut h_triplets = Vec::new();

        for i in 0..self.faces.len() {
            for j in 0..3 {
                let h = Halfedge::new(3 * i + j);

                let v_small = min(h.v_src(&self.faces), h.v_tgt(&self.faces));
                let v_large = max(h.v_src(&self.faces), h.v_tgt(&self.faces));
                h_triplets.push((v_small, v_large, h.index));
                self.halfedges.push(h);
            }
        }

        h_triplets.sort();

        let mut idx = 0;
        while idx < self.halfedges.len() {
            if h_triplets[idx].0 == h_triplets[idx+1].0 && h_triplets[idx].1 == h_triplets[idx+1].1  {
                self.halfedges[h_triplets[idx  ].2].h_opp = h_triplets[idx+1].2 as isize;
                self.halfedges[h_triplets[idx+1].2].h_opp = h_triplets[idx  ].2 as isize;
                idx += 1;
            } else {
                self.halfedges[h_triplets[idx].2].h_opp = -1;
            }
            idx += 1;
        }

        // h_outを計算
        // 境界頂点の場合は境界半辺を保存
        self.h_out.resize(self.vertices.len(), 0);
        for (i, h) in self.halfedges.iter().enumerate() {
            if self.halfedges[self.h_out[h.v_src(&self.faces)]].is_not_boundary() {
                self.h_out[h.v_src(&self.faces)] = i;
            }
        }

        // 境界半辺の h_opp に次の境界半辺を保存
        // boundary_h_1.h_opp = - boundary_h_2 - 1
        for &ho in &self.h_out {
            if self.halfedges[ho].is_boundary() {
                let vt = self.halfedges[ho].v_tgt(&self.faces);
                let out_h = self.h_out[vt] as isize;
                self.halfedges[ho].h_opp = -out_h - 1;
            }
        }
    }

    pub fn calc_face_normals(&mut self) {
        self.normal_face.resize(self.faces.len(), na::Vector3::zeros());
        for (i, n_f) in self.normal_face.iter_mut().enumerate() {
            let vn0 = self.normal_vertex[self.faces[i][0]];
            let vn1 = self.normal_vertex[self.faces[i][1]];
            let vn2 = self.normal_vertex[self.faces[i][2]];

            *n_f = (vn0 + vn1 + vn2).normalize();
        }
    }

    pub fn h_cw(&self, index: isize) -> isize {
        if index < 0 {
            index
        } else if self.halfedges[index as usize].h_opp < 0 {
            self.halfedges[index as usize].h_opp
        } else {
            self.halfedges[self.halfedges[index as usize].h_opp as usize].h_next() as isize
        }
    }

    pub fn h_ccw(&self, index: isize) -> isize {
        if index < 0 {
            index
        } else {
            self.halfedges[self.halfedges[index as usize].h_prev()].h_opp
        }
    }
}
