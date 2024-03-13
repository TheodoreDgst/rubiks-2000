#![allow(dead_code)] // This macros disable all the "unsed function/variable ..." warning, to simplify clarity during dev

use std::{ io, thread };

const N_THREAD: usize = 6;

use crate::cube::defs::*;
use crate::cube::enums::*;
use crate::cube::cube::Cube;

use std::fs::{ self, File };
use std::io::{ Read, Seek, SeekFrom, Write };

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
    for _ in 0..N_EDGE_ORI {
        let mut line = Vec::new();
        for _ in 0..10 {
            line.push(Move::Default);
        }
        table.push(line);
    }

    let mut cube = DEFAULT;

    // depends on the parent thread
    let mut moves_done = if id < N_THREAD / 2 {
        cube.edge_multiply(MOVE_B);
        vec![Move::B1]
    } else {
        cube.edge_multiply(MOVE_F);
        vec![Move::F1]
    };

    let moves_to_do = if id < N_THREAD / 2 {
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
    } else {
        [0, 1, 2, 3, 4, 5, 9, 10, 11, 12, 13, 14, 15, 16, 17]
    };

    let copy = cube;

    table[cube.get_flip()] = moves_done.clone();

    // handle which move do each thread
    for i_mv in ((id % 3) * 5)..(((id % 3) + 1) * 5) {
        let mv = ALL_MOVES[moves_to_do[i_mv]];
        cube.edge_multiply(mv);
        moves_done.push(Move::from(moves_to_do[i_mv]));
        if id >= 3 {
            println!("thread {} move {}", id, Move::from(moves_to_do[i_mv]));
        }
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

    for mov_seq in 0..N_EDGE_ORI {
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
    if depth >= MAX_DEPTH_PHASE_1 {
        return;
    }

    let code = cube.get_flip();

    if depth < move_count(&table[code]) {
        table[code] = moves_done.clone();
    }

    let prev = if depth >= 1 { *(&moves_done).get(depth - 1).unwrap() } else { Move::Default };

    let prev_2 = if depth >= 2 { *(&moves_done).get(depth - 2).unwrap() } else { Move::Default };

    // We use copy rather than juste inverse move because it's 2 times faster
    let base_cube = cube;

    for (i_mov, c_mov) in ALL_MOVES.iter().enumerate() {
        let mov = Move::from(i_mov);

        if
            (depth >= 1 && mov.is_same_slice(prev)) ||
            (depth >= 2 && mov.is_opposed_slice(prev) && mov.is_same_slice(prev_2))
        {
            continue;
        }

        cube.edge_multiply(*c_mov);
        moves_done.push(mov);

        create_table_vec_rec(cube, table, depth + 1, moves_done);

        cube = base_cube;

        moves_done.pop();
    }
}

pub fn create_table_1(file_path: &str) {
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

fn read_line_from_file_by_index(filename: &str, n: usize) -> io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();

    // Déplacer le curseur vers le début du fichier
    file.seek(SeekFrom::Start(0))?;

    // Itérer sur chaque ligne
    let mut line_count = 0;
    //let mut current_byte = 0;
    loop {
        let mut byte = [0; 1];
        file.read_exact(&mut byte)?;
        if byte[0] == b'|' {
            line_count += 1;
        }
        if line_count == n {
            buffer.push(byte[0]);
        } else if line_count == n + 1 {
            break;
        }
        //current_byte += 1;
    }

    Ok(buffer)
}

pub fn encode_solution(solution: Vec<Move>) -> u32 {
    let mut res = 0;
    for mv in solution {
        res += mv as u32;
        res *= 18;
    }
    res
}

fn get_solution_from_table(position: usize, data_size: usize, file_path: &str) -> Option<u32> {
    // Open the file
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            return None;
        } // Retourne None si l'ouverture du fichier échoue
    };

    // Calculer la position dans le fichier pour l'élément n
    let position = position * data_size; // Index basé sur 0 pour la position

    // Se déplacer à la position appropriée dans le fichier
    if let Err(_) = file.seek(SeekFrom::Start(position as u64)) {
        return None; // Retourne None si le déplacement dans le fichier échoue
    }

    // Lire les 3 octets correspondant à l'élément à partir de la position spécifiée
    let mut buffer = [0; 10];
    if let Err(_) = file.read_exact(&mut buffer) {
        return None; // Retourne None si la lecture des données échoue
    }

    // Convertir les octets en un entier (u32)
    let data = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], 0]);
    Some(data)
}

pub fn get_list_move_from_table(position: usize, file_path: &str) -> Vec<Move> {
    let bytes = read_line_from_file_by_index(file_path, position);

    let mut res = vec![];

    for b in bytes.unwrap() {
        if b < 18 {
            res.push(Move::from(b as usize));
        }
    }

    res
}

pub fn decode_solution(code: u32) -> Vec<Move> {
    let mut code = code;
    let mut res = Vec::new();
    while code != 0 {
        res.push(Move::from((code % 18) as usize));
        code /= 18;
    }
    res
}
