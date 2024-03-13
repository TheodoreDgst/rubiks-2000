#![allow(dead_code)] // This macros disable all the "unsed function/variable ..." warning, to simplify clarity during dev

mod cube;
mod table;
use crate::cube::enums::Move;

fn print_move_sequenve(move_sequence: &[Move]) {
    for mv in move_sequence {
        print!("{} ", mv);
    }
    println!();
}

// Test module
#[cfg(test)]
mod tests;

fn main() {

}
