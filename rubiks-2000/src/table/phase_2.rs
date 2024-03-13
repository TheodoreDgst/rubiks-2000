#![allow(dead_code)] // This macros disable all the "unsed function/variable ..." warning, to simplify clarity during dev

use std::{ io, thread };

const N_THREAD: usize = 8;

use crate::cube::defs::*;
use crate::cube::enums::*;
use crate::cube::cube::Cube;

use std::fs::{ self, File };
use std::io::{ Read, Seek, SeekFrom, Write };

fn are_midle_edges_placed(edges: &[Edge]) -> bool {
    edges[0] == Edge::UR ||
        edges[0] == Edge::UL ||
        edges[0] == Edge::DR ||
        (edges[0] == Edge::DL && edges[2] == Edge::UR) ||
        edges[2] == Edge::UL ||
        edges[2] == Edge::DR ||
        (edges[2] == Edge::DL && edges[4] == Edge::UR) ||
        edges[4] == Edge::UL ||
        edges[4] == Edge::DR ||
        (edges[4] == Edge::DL && edges[6] == Edge::UR) ||
        edges[6] == Edge::UL ||
        edges[6] == Edge::DR ||
        edges[6] == Edge::DL
}

fn encode(cube: &Cube) -> usize {

    todo!()
}

/// Count the number of move in a Vec of move
/// (basically, it's just the len plus one for every 2*Move)
fn move_count(moves: &[Move]) -> usize {
    let mut len = moves.len();
    for mv in moves {
        if (*mv as usize) % 3 == 1 {
            len += 1;
        }
    }
    len
}

/// Create the thread i and begin the search process.
/// Returns the table associated
fn create_thread_table_vec(id: usize) -> Vec<Vec<Move>> {
    // the final table
    let mut table: Vec<Vec<Move>> = Vec::new();

    // initialize the table
    for _ in 0..CARD_G_2 {
        let mut line = Vec::new();
        for _ in 0..14 {
            line.push(Move::Default);
        }
        table.push(line);
    }

    let mut cube = DEFAULT;

    let copy = cube;

    let moves_to_do = [0, 1, 2, 4, 6, 7, 8, 9, 10, 11, 13, 15, 16, 17];

    let mut moves_done = Vec::new();

    // handle which move do each thread
    for i_mv in (id % 4) * 4..((id % 4) + 1) * 4 {
        let mv = ALL_MOVES[moves_to_do[i_mv]];
        cube.edge_multiply(mv);
        moves_done.push(Move::from(moves_to_do[i_mv]));
        //println!("thread {} move {}", id, Move::from(moves_to_do[i_mv]));
        create_table_vec_rec(cube, &mut table, 2, &mut moves_done);
        cube = copy;
        moves_done.pop();
    }
    table
}

/// This function create the whole table with multi threading, return the table
/// The two mains thread apply F or B move and then start the search by divinding the tree into 2 others subthread
fn create_table_vec() -> Vec<Vec<Move>> {
    let mut thread_vec = Vec::new();

    // create and start the threads
    for id in 0..N_THREAD {
        let thread = thread::spawn(move || { create_thread_table_vec(id) });
        thread_vec.push(thread);
    }

    // collect thread result
    let mut result_vec = Vec::new();
    for thread in thread_vec {
        result_vec.push(thread.join().unwrap());
    }

    // merge the results
    let mut final_table = Vec::new();

    for mov_seq in 0..CARD_G_2 {
        let mut tmp_table = Vec::new();
        for sub_table in 0..N_THREAD {
            tmp_table.push(result_vec[sub_table][mov_seq].clone());
        }

        // Take the shortest sequence of move
        let mut min = tmp_table[0].clone();

        for seq in tmp_table {
            if move_count(&seq) < move_count(&min) {
                min = seq;
            }
        }

        final_table.push(min);
    }

    final_table
}

/// Recursive function used by each thread
fn create_table_vec_rec(
    mut cube: Cube,
    table: &mut Vec<Vec<Move>>,
    depth: usize,
    moves_done: &mut Vec<Move>
) {
    if depth >= MAX_DEPTH_PHASE_2 {
        return;
    }

    let code = cube.get_twist();

    if are_midle_edges_placed(&cube.get_ep()) && depth < move_count(&table[code]) {
        table[code] = moves_done.clone();
    }

    let prev = if depth >= 1 { *(&moves_done).get(depth - 1).unwrap() } else { Move::Default };

    let prev_2 = if depth >= 2 { *(&moves_done).get(depth - 2).unwrap() } else { Move::Default };

    // We use copy rather than juste inverse move because it's 2 times faster
    let base_cube = cube;

    let moves_to_do = [0, 1, 2, 4, 6, 7, 8, 9, 10, 11, 13, 15, 16, 17];

    for i_mov in moves_to_do {
        let c_mov = ALL_MOVES[i_mov];
        let mov = Move::from(i_mov);

        if
            (depth >= 1 && mov.is_same_slice(prev)) ||
            (depth >= 2 && mov.is_opposed_slice(prev) && mov.is_same_slice(prev_2))
        {
            continue;
        }

        cube.edge_multiply(c_mov);
        moves_done.push(mov);

        create_table_vec_rec(cube, table, depth + 1, moves_done);

        cube = base_cube;

        moves_done.pop();
    }
}

pub fn create_table_2(file_path: &str) {
    if fs::metadata(file_path).is_ok() {
        println!("The table {} already exists.", file_path);
        return;
    }

    println!("Table 1 creation :");

    let table = create_table_vec();

    let mut file = File::create(file_path).unwrap();

    for mov_sequence in table {
        let bytes: Vec<u8> = mov_sequence
            .iter()
            .map(|x| *x as u8)
            .collect();

        file.write_all(&bytes).unwrap();
        // 124 = '|' because \n = 10 which is < 18 so its a move
        file.write_all(&[124]).unwrap();
    }
}
