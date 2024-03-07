use std::fs::{ self, File };
use std::io::{ self, Read, Write };

//use crate::cube::enums::Move;
use crate::cube::defs::*;

use crate::cube::cube::*;
use crate::cube::enums::Move;

/// WARNING!: this file is in devlopment, at terme it will be used to create the first table (to go into sub group 1)
/// and to perform the phase 1 of the algorithme

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

    pub fn find_solution_to_g1(&self, cube: Cube) -> Queue<Move> {
        let mut solution = Queue::new();
        let found = self.__dfs(cube, 0, &mut solution, &mut (MAX_DEPTH_PHASE_1 + 1));

        if found {
            println!("Solution trouvée");
        } else {
            println!("Solution non trouvée");
        }

        solution
    }

    // Private method for depth-first search to find a solution to G1.
    fn __dfs(
        &self,
        mut cube: Cube,
        depth: u8,
        moves_done: &mut Queue<Move>,
        min_depth: &mut u8
    ) -> bool {
        if depth > *min_depth {
            return false;
        }
        if cube == DEFAULT {
            *min_depth = depth;
            println!("new min depth = {}", *min_depth);
            return true;
        }

        let copy = cube;

        // Iterate over base moves and perform depth-first search
        for (i_mov, mov) in ALL_MOVES.iter().enumerate() {
            cube.multiply(*mov);
            // println!("rec - {}", Move::from(i_mov).to_string());
            if self.__dfs(cube, depth + 1, moves_done, min_depth) {
                moves_done.enqueue(i_mov.into());
            }
            cube = copy;
        }

        false
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

#[derive(Debug, Clone, PartialEq)]
pub struct Queue<T> {
    pub items: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { items: Vec::new() }
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() { None } else { Some(self.items.remove(0)) }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}
