use std::fs::{self, File};
use std::io::{self, Read, Write};

use crate::cube::enums::Move;
use crate::{ALL_MOVES, BASE_MOVES, CO_DEFAULT, CP_DEFAULT, EP_DEFAULT, MAX_DEPTH_PHASE_1};

use crate::cube::cube::Cube;

pub struct TablePhase1 {
    file_path: String,
}

impl TablePhase1 {
    pub fn new(path: String) -> Self {
        Self { file_path: path }
    }

    pub fn generate(&self) {
        let file_path = &self.file_path;
        let mut file = File::create(file_path).unwrap();

        if fs::metadata(file_path).is_ok() {
            println!("The table {} already exists.", file_path);
            //return;
        }

        for i in 0..2048 {
            let bytes_to_write: [u8; 11] = u16_to_binary_array(i);
            for &byte in bytes_to_write.iter() {
                // TODO: check le result
                file.write_all(&[byte]).unwrap();
            }
            file.write_all(b"\n").unwrap();
        }
    }

    pub fn find_solution_to_g1(&self, _edges_orientations: u16) -> Option<Vec<Cube>> {
        let cube = Cube::new(
            CP_DEFAULT,
            CO_DEFAULT,
            EP_DEFAULT,
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        );

        let solution = self.__dfs(cube, 0, &mut Vec::new());

        match solution {
            Some(_) => {
                println!("solution teouvée!!!!");
            }
            None => {
                println!("solution non trouvée!!!!");
            }
        }

        solution
    }

    fn __dfs(&self, mut cube: Cube, depth: u8, moves_done: &mut Vec<Cube>) -> Option<Vec<Cube>> {
        if cube.is_solved_phase_1() {
            return Some(moves_done.to_vec());
        } else if depth > MAX_DEPTH_PHASE_1 {
            return None;
        }

        for mov in BASE_MOVES.iter() {
            cube.multiply(*mov);
            moves_done.push(*mov); //PB mdr
            match self.__dfs(cube, depth + 1, moves_done) {
                Some(solution_vec) => return Some(solution_vec),
                None => {
                    moves_done.pop();
                }
            }
        }

        None
    }

    pub fn read(self) -> io::Result<()> {
        // No sure about th Result ...
        let mut file = File::open(self.file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        for byte in buffer {
            println!("{}", byte);
        }

        Ok(())
    }
}

pub fn u16_to_binary_array(n: u16) -> [u8; 11] {
    let mut result = [0; 11];

    for i in 0..11 {
        let bit = (n >> (11 - i)) & 1;
        result[i] = bit as u8;
    }

    result
}
