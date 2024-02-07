mod cube;
use std::vec;

use cube::cube::Cube;

use crate::cube::cube::Color;


fn main() {
    let mut c = Cube::new_filled(3);
    c.dbg();
    c.shuffle(100);
    c.dbg()
}
