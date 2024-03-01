use super::cube::*;
use super::defs::*;
use super::enums::*;
#[derive(Debug, Clone, Copy)]
pub struct FaceCube {
    facelets: [Color; 54],
}

impl FaceCube {
    pub fn new() -> Self {
        let mut facelets = [Color::B; 54];
        for i in 0..9 {
            facelets[i] = Color::U;
        }
        for i in 9..18 {
            facelets[i] = Color::R;
        }
        for i in 18..27 {
            facelets[i] = Color::F;
        }
        for i in 27..36 {
            facelets[i] = Color::D;
        }
        for i in 35..45 {
            facelets[i] = Color::L;
        }
        FaceCube { facelets }
    }
    
    pub fn to_string(self) -> String {
        let mut res=String::new();
        for i in 0..54 {
            match self.facelets[i]{
                Color::B => res.push('B'),
                Color::R => res.push('R'),
                Color::U => res.push('U'),
                Color::F => res.push('F'),
                Color::L => res.push('L'),
                Color::D => res.push('D'),
            }
        }
        res
    }
    
    pub fn to_cubie_cube(self) -> Cube {
        let mut cp = [Corner::URF; 8];
        let mut co: [u8; 8] = [1; 8];
        let mut ep = [Edge::UB; 12];
        let mut eo = [1; 12];
        for i in 0..N_CORNERS {
            let fac = CORNER_FACELET[i as usize];
            let mut ori = 0;
            for or in 0..3 {
                if self.facelets[fac[ori] as usize] == Color::U
                    || self.facelets[fac[ori] as usize] == Color::D
                {
                    ori = or;
                    break;
                }
            }
            let col1 = self.facelets[fac[(ori + 1) % 3] as usize];
            let col2 = self.facelets[fac[(ori + 2) % 3] as usize];
            for j in 0..N_CORNERS {
                let col = CORNER_COLOR[j as usize];
                if col1 == col[1] && col2 == col[2] {
                    cp[i as usize] = Corner::from(j);
                    co[i as usize] = ori as u8;
                    break;
                }
            }
        }
        for i in 0..N_EDGES {
            for j in 0..N_EDGES {
                if self.facelets[EDGE_FACELET[i as usize][0] as usize] == EDGE_COLOR[j as usize][0]
                    && self.facelets[EDGE_FACELET[i as usize][1] as usize]
                        == EDGE_COLOR[j as usize][1]
                {
                    ep[i as usize] = Edge::from(j);
                    eo[i as usize] = 0;
                    break;
                }
                if self.facelets[EDGE_FACELET[i as usize][0] as usize] == EDGE_COLOR[j as usize][1]
                    && self.facelets[EDGE_FACELET[i as usize][1] as usize]
                        == EDGE_COLOR[j as usize][0]
                {
                    ep[i as usize] = Edge::from(j);
                    eo[i as usize] = 1;
                    break;
                }
            }
        }
        Cube::new(cp, co, ep, eo)
    }

    pub fn set_facelet(&mut self, index: usize, new_color: Color) {
        self.facelets[index] = new_color;
    }
}
