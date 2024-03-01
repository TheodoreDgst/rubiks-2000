use crate::cube::cube::Cube;

impl Cube {
    pub fn is_solved_phase_1(&self) -> bool {
        // NOTE: maybe we should test only the 11th firsts??
        self.get_eo() == [0; 12]
    }
}
