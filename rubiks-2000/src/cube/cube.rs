use super::defs::*;
use super::enums::{Color, Corner, Edge};
use super::face_cube::*;
use std::fmt; // Usef for impl display

#[derive(Debug, Clone, Copy)]
pub struct Cube {
    cp: [Corner; 8],
    co: [u8; 8],
    ep: [Edge; 12],
    eo: [u8; 12],
}

impl Cube {
    /// Create a new Cube.
    /// # Args :
    /// * `cp` : The corners permutations (positions) of the cube.
    /// * `co` : The corners orientations of the cube.
    /// * `ep` : The edges permutations (positions) of the cube.
    /// * `eo` : The edges orientations of the cube.
    pub const fn new(cp: [Corner; 8], co: [u8; 8], ep: [Edge; 12], eo: [u8; 12]) -> Self {
        // hein?? pk elle peut etre const elle?? fada
        Self { cp, co, ep, eo }
    }

    /// Create a new Cube solved
    pub const fn new_default() -> Self {
        let cp = CP_DEFAULT;
        let co = CO_DEFAULT;
        let ep = EP_DEFAULT;
        let eo = EO_DEFAULT;
        Self { cp, co, ep, eo }
    }

    /// Returns the corner permutations of the cube
    pub fn get_cp(&self) -> [Corner; 8] {
        self.cp
    }

    /// Returns the corner orientations of the cube
    pub fn get_co(&self) -> [u8; 8] {
        self.co
    }

    /// Returns the edge permutations of the cube
    pub fn get_ep(&self) -> [Edge; 12] {
        self.ep
    }

    /// Returns the edge orientations of the cube
    pub fn get_eo(&self) -> [u8; 12] {
        self.eo
    }

    pub fn corner_multiply(&mut self, other: Cube) {
        // Multiply this cubie cube with another cubie cube "other", restricted to the corners. Does not change other.
        let mut new_corner_perm = [0; 8];
        let mut new_corner_ori = [0; 8];
        let mut current_ori = 0;

        for c in 0..8 {
            new_corner_perm[c] = self.cp[b.cp[c] as usize] as u8;
            let ori_a = self.co[b.cp[c] as usize] as i8;
            let ori_b = b.co[c] as i8;
            if ori_a < 3 && ori_b < 3 {
                // two regular cubes
                current_ori = ori_a + ori_b;
                if current_ori >= 3 {
                    current_ori -= 3;
                }
            } else if ori_a < 3 && ori_b >= 3 {
                // cube b is in a mirrored state
                current_ori = ori_a + ori_b;
                if current_ori >= 6 {
                    current_ori -= 3;
                }
            } else if ori_a >= 3 && ori_b < 3 {
                // cube a is in a mirrored state
                current_ori = ori_a - ori_b;
                if current_ori < 3 {
                    current_ori += 3;
                }
            } else if ori_a >= 3 && ori_b >= 3 {
                // if both cubes are in mirrored states
                current_ori = ori_a - ori_b;
                if current_ori < 0 {
                    current_ori += 3;
                }
            }
            new_corner_ori[c] = current_ori;
        }

        for c in 0..8 {
            self.cp[c] = new_corner_perm[c].into();
            self.co[c] = new_corner_ori[c] as u8;
        }
    }

    pub fn edge_multiply(&mut self, other: Cube) {
        // Multiply this Cubie Cube with another Cubie Cube 'other', restricted to the edges. Does not change 'other'.
        let mut new_edge_permutation: Vec<u8> = vec![0; 12];
        let mut new_edge_orientation: Vec<u8> = vec![0; 12];
        for edge in 0..12 {
            new_edge_permutation[edge] = self.ep[other.ep[edge] as usize] as u8;
            // Calculate the new edge orientation based on the orientations of 'other' and the current cube
            new_edge_orientation[edge] = (other.eo[edge] + self.eo[other.eo[edge] as usize]) % 2;
        }
        // Update the edge permutation and orientation of the current cube
        for edge in 0..12 {
            self.ep[edge] = new_edge_permutation[edge].into();
            self.eo[edge] = new_edge_orientation[edge];
        }
    }

    pub fn multiply(&mut self, other: Cube) {
        //apply the differente move of the cube other to current cube
        self.corner_multiply(other);
        self.edge_multiply(other);
    }

    /// Return the 'twist' of cube which mean the orientation of all its (8) corners represnted by one number between 0 and 2187.
    /// 3 orientations, 8 corners but we ignore one because it's not possible that only one is twisted
    /// => 3^7 = 2187 possibilities
    /// Do not confuse with 'flip'
    /// We ignore the last one (7th index)
    pub fn get_twist(self) -> u16 {
        let mut ret = 0;
        // from 0 to 7 because of the rubik's cube law (it's not possible that only one corner is twisted)
        for i in 0..7 {
            ret = 3 * ret + self.co[i] as u16;
        }
        return ret;
    }

    pub fn set_twist(&mut self, mut twist: u16) {
        let mut twist_parity = 0;
        for i in (0..7).rev() {
            self.co[i] = (twist % 3) as u8;
            twist_parity += self.co[i];
            twist /= 3;
        }
        self.co[7] = (3 - twist_parity % 3) % 3;
    }

    pub fn set_flip(&mut self, mut flip: u16) {
        let mut flip_parity = 0;
        for i in (0..11).rev() {
            self.eo[i] = (flip % 2) as u8;
            flip_parity += self.eo[i];
            flip /= 2
        }
        self.eo[11] = (2 - flip_parity % 2) % 2;
    }

    /// Return the 'flip' of cube which means the orientation of all its (12) edgesq represented by one number between 0 and 2048
    /// 2 orientation, 12 corners but we ignore one because it's not possible that only one is twisted
    /// => 2^11 = 2048 possibilities
    /// Do not confuse with 'twist'
    /// We ignore the last one (11th index )
    pub fn get_flip(self) -> u16 {
        let mut ret = 0;
        for i in 0..11 {
            ret = 2 * ret + self.eo[i] as u16;
        }
        return ret;
    }

    pub fn to_facelet_cube(self) -> FaceCube {
        //"""Return a facelet representation of the cube."""
        let mut face_cube = FaceCube::new();
        for color in 0..N_COLORS {
            let corner = self.cp[color as usize]; //# corner j is at corner position i
            let orientation = self.co[color as usize]; //# orientation of C j at position i
            for k in 0..3 {
                let index =
                    CORNER_FACELET[color as usize][((k + orientation) % 3) as usize] as usize;
                let new_color = CORNER_COLOR[corner as usize][k as usize];
                face_cube.set_facelet(index, new_color);
            }
        }

        for edge in 0..N_EDGES {
            let j = self.ep[edge as usize]; //# similar for Es
            let orientation = self.eo[edge as usize];
            for k in 0..2 {
                let index = EDGE_FACELET[edge as usize][((k + orientation) % 2) as usize] as usize;
                let new_color = EDGE_COLOR[j as usize][k as usize];
                face_cube.set_facelet(index, new_color);
            }
        }
        face_cube
    }

    pub fn corner_parity(self) -> u8 {
        //"""Give the parity of the corner permutation."""
        let mut parity = 0;
        for corner in (0..N_CORNERS).rev() {
            for permutation in (corner..=0).rev() {
                if self.cp[permutation as usize] as u8 > self.cp[corner as usize] as u8 {
                    parity += 1;
                }
            }
        }
        parity % 2
    }

    pub fn edge_parity(self) -> u8 {
        //"""Give the parity of the edge permutation. A solvable cube has the same corner and edge parity."""
        let mut parity = 0;
        for edge in (0..N_EDGES).rev() {
            for permutation in (0..edge).rev() {
                if self.ep[permutation as usize] as u8 > self.ep[edge as usize] as u8 {
                    parity += 1;
                }
            }
        }
        parity % 2
    }

    pub fn verify(&self) -> bool {
        //"""Check if cubiecube is valid."""
        let mut edge_count = [0; 12];
        for i in 0..N_EDGES {
            edge_count[self.ep[i as usize] as usize] += 1;
        }
        for edge in 0..N_EDGES {
            if edge_count[edge as usize] != 1 {
                println!("Some edges are invalid ...");
                return false;
            }
        }

        let mut verify_edge = 0;
        for edge in 0..N_EDGES {
            verify_edge += self.eo[edge as usize];
        }
        if verify_edge % 2 != 0 {
            println!("Total edge flip is wrong ...");
            return false;
        }

        let mut corner_count = [0; 8];
        for corner in 0..N_CORNERS {
            corner_count[self.cp[corner as usize] as usize] += 1;
        }
        for corner in 0..N_CORNERS {
            if corner_count[corner as usize] != 1 {
                println!("Some corners are invalid ...");
                return false;
            }
        }

        let mut verify_corner = 0;
        for corner in 0..N_COLORS {
            verify_corner += self.co[corner as usize];
        }

        if verify_corner % 3 != 0 {
            println!("Total corner flip is wrong...");
            return false;
        }

        if self.edge_parity() != self.corner_parity() {
            println!("Wrong egde and corner parity...");
            return false;
        }

        return true;
    }
}

/// Display implementation for CubieCube (to_string() method)
/// # Format :
/// (corner, orientation) ... (edge, orientation)
impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::new();
        for corner in 0..8 {
            // NOTE: how to use the enum here rather than 0..8 ??
            // The 8 corners
            string += format!("({},{})", self.cp[corner], self.co[corner]).as_str();
        }
        string += "|";
        for edge in 0..12 {
            // The 12 edges
            string += format!("({},{})", self.ep[edge], self.eo[edge]).as_str();
        }
        write!(f, "{}", string)
    }
}

/// Compare row CubieCube.
/// Allow this:
/// if cube_a == cube_b {...}
impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        if self.cp == other.cp && self.co == other.co && self.ep == other.ep && self.eo == other.eo
        {
            return true;
        }
        false
    }
}
