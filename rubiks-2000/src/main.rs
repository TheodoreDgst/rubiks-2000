#![allow(dead_code)] // This macros disable all the "unsed function/variable ..." warning, to simplify clarity during dev

mod cube;
mod table;

use cube::{ defs::*, enums::Move::* };
use table::table::*;

// Test module
#[cfg(test)]
mod tests;

fn main() {
    let mut cube = DEFAULT;

    cube.multiply(MOVE_F);

    let table = TablePhase1::new(String::from("taable"));
    let mut solution = table.find_solution_to_g1(cube);

    while !solution.is_empty() {
        let mv = solution.dequeue();
        match mv {
            Some(m) => println!("{}", m.to_string()),
            None => {
                break;
            }
        }
    }

    let mut c = DEFAULT;

    let m = [F2, U1, U1, R3, U1, U1];

    for mv in m {
        c.multiply(ALL_MOVES[mv as usize]);
    }

    assert_eq!(DEFAULT, c);
}
