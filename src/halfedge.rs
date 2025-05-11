use crate::mesh::Mesh;

// pub enum HEType {
//     Internal(usize),
//     Boundary(usize),
//     None,
// }
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Halfedge {
    pub index: usize,
    pub h_opp: isize,
}

impl Halfedge {
    pub fn new(he_idx: usize) -> Halfedge {
        Halfedge { index: he_idx, h_opp: 0 }
    }
    
    pub fn face(&self) -> usize {
        self.index / 3
    }

    pub fn h_next(&self) -> usize {
        if self.index % 3 == 2 {
            self.index - 2
        } else {
            self.index + 1
        }
    }

    pub fn h_prev(&self) -> usize {
        if self.index % 3 == 0 {
            self.index + 2
        } else {
            self.index - 1
        }
    }

    pub fn v_src(&self, faces: &Vec<na::Vector3<usize>>) -> usize {
        faces[self.face()][(self.index + 1) % 3]
    }

    pub fn v_tgt(&self, faces: &Vec<na::Vector3<usize>>) -> usize {
        faces[self.face()][(self.index + 2) % 3]
    }

    pub fn is_boundary(&self) -> bool {
        self.h_opp < 0
    }

    pub fn is_not_boundary(&self) -> bool {
        ! self.is_boundary()
    }

    pub fn norm(&self, mesh: &Mesh) -> f64 {
        let x0 = mesh.vertices[self.v_src(&mesh.faces)];
        let x1 = mesh.vertices[self.v_tgt(&mesh.faces)];
        (x0 - x1).norm()
    }

}