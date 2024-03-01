#![allow(dead_code)] // This macros disable all the "unsed function/variable ..." warning, to simplify clarity during dev

mod cube;
use cube::cube::Cube;
use cube::defs::*;
mod table;
//use table::phase1::*;
use table::table::*;

// Test module
#[cfg(test)]
mod tests;

fn main() {
    let mut c = Cube::new_default();
    c.set_twist(1);

    //c.multiply(MOVE_B);

    let t = TablePhase1::new(String::from("taable"));

    let s = t.find_solution_to_g1(c.get_flip());
}
