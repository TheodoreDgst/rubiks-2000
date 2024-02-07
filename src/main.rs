mod cube;
use cube::cube::Cube;

use crate::cube::cube::Color;


fn main() {
    let mut c = Cube::new(3);
    c.dbg();
    c.faces[0][1] = Color::Green;
    c.faces[0][3] = Color::Blue;
    c.dbg();
    let v = c.get_row(0, 0);
    dbg!(v);
}
