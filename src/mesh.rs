use crate::halfedge::Halfedge;
use std::collections::BTreeMap;

pub struct Mesh {
    pub vertices: Vec<na::Vector3<f64>>,
    pub faces: Vec<na::Vector3<usize>>,
    pub normal_vertex: Vec<na::Vector3<f64>>,

    pub halfedges: Vec<Halfedge>,
    pub h_out: Vec<usize>
}

impl Mesh {
    pub fn new() -> Mesh {
        let vertices = Vec::new();
        let faces = Vec::new();
        let normal_vertex = Vec::new();
        let halfedges = Vec::new();
        let h_out = Vec::new();
        Mesh {
            vertices,
            faces,
            normal_vertex,
            halfedges,
            h_out
        }
    }

    pub fn construct_halfedge_list(&mut self) {
        // h_oppを計算
        let mut he_map = BTreeMap::new();

        for i in 0..self.faces.len() {
            for j in 0..3 {
                let mut he = Halfedge::new(3 * i + j);

                let key      = (he.v_src(&self.faces), he.v_tgt(&self.faces));
                let key_swap = (he.v_tgt(&self.faces), he.v_src(&self.faces));

                match he_map.get(&key_swap) {
                    Some(&opp_idx) => {
                        he.h_opp = opp_idx as isize;
                        self.halfedges[opp_idx as usize].h_opp = (3 * i + j) as isize;
                    }
                    None => {
                        he_map.insert(key, 3 * i + j);
                    }
                }

                self.halfedges.push(he);
            }
        }

        // h_outを計算
        // 境界頂点の場合は境界半辺を保存
        self.h_out.resize(self.vertices.len(), 0);
        for (i, he) in self.halfedges.iter().enumerate() {
            if self.halfedges[self.h_out[he.v_src(&self.faces)]].is_not_boundary() {
                self.h_out[he.v_src(&self.faces)] = i;
            }
        }

        // 境界半辺の h_opp に次の境界半辺を保存
        // boundary_he_1.h_opp = - boundary_he_2 - 1
        for &ho in &self.h_out {
            if self.halfedges[ho].is_boundary() {
                let vt = self.halfedges[ho].v_tgt(&self.faces);
                let out_he = self.h_out[vt] as isize;
                self.halfedges[ho].h_opp = -out_he - 1;
            }
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
