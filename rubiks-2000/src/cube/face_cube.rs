use super::cube::*;
use super::defs::*;
use super::enums::*;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FaceCube {
    facelets: [Color; 54],
}

impl FaceCube {
    pub fn new() -> Self {
        let mut facelets = [Color::B; 54];
        // Assign colors to the facelets based on their position in the solve cube
        for up in 0..9 {
            facelets[Up] = Color::U;
        }
        for right in 9..18 {
            facelets[right] = Color::R;
        }
        for front in 18..27 {
            facelets[front] = Color::F;
        }
        for down in 27..36 {
            facelets[down] = Color::D;
        }
        for left in 36..45 {
            facelets[left] = Color::L;
        }
        FaceCube { facelets }
    }
    pub fn from_string(str_cube: &str) -> Self {
        //transform a string of the cube into this struct
        if str_cube.len() < 54 {
            panic!("pas le bon nombre de facelets");
        }
        if str_cube.len() > 54 {
            panic!("pas le bon nombre de facelets");
        }

        let mut facelets = [Color::B; 54];
        let mut cpt = [0; 6];
        for (index,color) in str_cube.char_indices() {
            match color {
                'U' => {
                    facelets[index] = Color::U;
                    cpt[0] += 1
                }
                'B' => {
                    facelets[index] = Color::B;
                    cpt[1] += 1
                }
                'F' => {
                    facelets[index] = Color::F;
                    cpt[2] += 1
                }
                'D' => {
                    facelets[index] = Color::D;
                    cpt[3] += 1
                }
                'L' => {
                    facelets[i.0] = Color::L;
                    cpt[4] += 1
                }
                'R' => {
                    facelets[i.0] = Color::R;
                    cpt[5] += 1
                }
                _ => panic!("pas les bonnes lettres"),
            }
        }
        // verify if the correct number of each color was provide
        for i in 0..6 {
            if cpt[i] != 9 {
                panic!("probleme de couleur");
            }
        }
        FaceCube { facelets }
    }
    pub fn to_string(self) -> String {
        //inverse of the previous fonction
        let mut res = String::new();
        for i in 0..54 {
            match self.facelets[i] {
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
        //transform this struct into the cube struct with the rotation of corner and edge and their position
        let mut cp = [Corner::URF; 8];
        let mut co: [u8; 8] = [1; 8];
        let mut ep = [Edge::UB; 12];
        let mut eo = [1; 12];
        for i in 0..N_CORNERS {
            let fac = CORNER_FACELET[i as usize];
            let mut ori = 0;
            for diff_ori in 0..3 {
                if self.facelets[fac[diff_ori] as usize] == Color::U
                    || self.facelets[fac[diff_ori] as usize] == Color::D
                {
                    ori = diff_ori;
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
