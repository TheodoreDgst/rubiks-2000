use crate::cube::cube::Cube;

/// WARNING!: this file is in devlopment, at terme it will be used to create the first table (to go into sub group 1)
/// and to perform the phase 1 of the algorithme


impl Cube {
    pub fn is_solved_phase_1(&self) -> bool {
        // NOTE: maybe we should test only the 11th firsts??
        self.get_eo() == [0; 12]
    }
}
