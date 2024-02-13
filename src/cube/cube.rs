use rand::Rng;
use std::fmt;
use std::vec;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Red,
    Orange,
    Blue,
    Yellow,
    Green,
    White,
    Empty,
}

impl Color {
    fn pretty_print(&self) {
        let code = match self {
            Self::Blue => "\x1b[34m",
            Self::White => "\x1b[37m",
            Self::Green => "\x1b[32m",
            Self::Red => "\x1b[31m",
            Self::Orange => "\x1b[38;5;208m",
            Self::Yellow => "\x1b[33m",
            _ => "\x1b[30m",
        };
        let reset_code = "\x1b[0m"; // Code pour réinitialiser la couleur à la couleur par défaut
        print!("{}██{}", code, reset_code);
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match self {
            Color::Red => "R",
            Color::Orange => "O",
            Color::Blue => "B",
            Color::Yellow => "Y",
            Color::Green => "G",
            Color::White => "W",
            Color::Empty => "E",
        };
        write!(f, "{}", string_representation)
    }
}

#[derive(Clone, Debug)]
pub struct Cube {
    size: usize,
    faces: Vec<Vec<Color>>,
}

impl Cube {
    /// This function create and return a cube filled with empty cases
    ///
    /// # Args
    /// * '_size' - the size of the cube
    pub fn new_empty(_size: usize) -> Self {
        Cube {
            size: _size,
            faces: vec![vec![Color::Empty; _size * _size]; _size],
        }
    }

    /// This function create and return a cube filled with the classical colors of
    /// a rubik's cube
    ///
    /// # Args
    /// * '_size' - the size of the cube
    pub fn new_filled(_size: usize) -> Self {
        let size = _size;
        let green_face = vec![Color::Green; size * size];
        let white_face = vec![Color::White; size * size];
        let orange_face = vec![Color::Orange; size * size];
        let yellow_face = vec![Color::Yellow; size * size];
        let red_face = vec![Color::Red; size * size];
        let blue_face = vec![Color::Blue; size * size];
        let faces = vec![
            white_face,
            blue_face,
            red_face,
            green_face,
            orange_face,
            yellow_face,
        ];
        Cube { size, faces: faces }
    }

    /// This function return the size of the cube
    pub fn get_size(&self) -> usize {
        self.size
    }

    /// This function draw the cube in the console
    pub fn dbg(&self) {
        println!();
        for (j, case) in self.faces[1].iter().enumerate() {
            if j % self.size == 0 {
                println!();
                let space = " ".repeat(self.size * 2);
                print!("{}", space);
            }
            case.pretty_print();
        }
        println!();

        for j in 0..self.size {
            for f in [2, 0, 4, 5] {
                for i in 0..self.size {
                    self.faces[f][i + j * self.size].pretty_print();
                }
            }
            println!();
        }
        for (j, case) in self.faces[3].iter().enumerate() {
            if j % self.size == 0 {
                if j != 0 {
                    println!();
                }
                let space = " ".repeat(self.size * 2);
                print!("{}", space);
            }
            case.pretty_print();
        }

        println!();
    }

    /// This function return a Vector of Color composing a certain colon on a face
    /// Color are copied, it's not ref.
    ///
    /// # Args
    /// * 'face' - the index of the face
    /// * 'col' - the index of the colon
    ///
    /// # Return
    ///
    /// The colors composing this colonn in a vector
    fn get_col(&self, face: usize, col: usize) -> Vec<Color> {
        let mut res = Vec::new();

        for (index, case) in self.faces[face].iter().enumerate() {
            if index % self.size == col {
                res.push(*case);
            }
        }

        res
    }

    /// This function return a Vector of Color composing a certain row on a face
    /// Color are copied, it's not ref.
    ///
    /// # Args
    /// * 'face' - the index of the face
    /// * 'row' - the index of the row
    ///
    /// # Return
    ///
    /// The colors composing this row in a vector
    fn get_row(&self, face: usize, row: usize) -> Vec<Color> {
        let mut res = Vec::new();
        let stop = self.size * (row + 1);
        for (index, case) in self.faces[face].iter().enumerate().skip(self.size * row) {
            if index < stop {
                res.push(*case);
            }
        }
        res
    }

    /// This function return a Vector of Color composing a certain colon on a face
    /// Before, it fill the colonn of this face with the colors passed in parameter
    /// Color are copied, it's not ref.
    ///
    /// # Args
    /// * 'face' - the index of the face
    /// * 'col' - the index of the colon to modify and return
    /// * 'rep' - the vector to use to replace the values in the colon
    ///
    /// # Return
    ///
    /// The colors composing this colonn (before it was changed) in a vector
    fn fill_col(&mut self, face: usize, col: usize, rep: Vec<Color>) -> Vec<Color> {
        let mut res = Vec::new();
        for i in 0..self.size {
            let acc = self.faces[face][col + i * self.size];
            res.push(acc);
            self.faces[face][col + i * self.size] = rep[i];
        }
        res
    }

    /// This function return a Vector of Color composing a certain row on a face
    /// Before, it fill the row of this face with the colors passed in parameter
    /// Color are copied, it's not ref.
    ///
    /// # Args
    /// * 'face' - the index of the face
    /// * 'row' - the index of the row to modify and return
    /// * 'rep' - the vector to use to replace the values in the row
    ///
    /// # Return
    ///
    /// The colors composing this row (before it was changed) in a vector
    fn fill_row(&mut self, face: usize, row: usize, rep: Vec<Color>) -> Vec<Color> {
        let mut res = Vec::new();
        for i in 0..self.size {
            let acc = self.faces[face][row * self.size + i];
            res.push(acc);
            self.faces[face][row * self.size + i] = rep[i];
        }
        res
    }

    /// This function push a colon of a cube verticaly (without rotatning faces if needed)
    /// So this is not a complete move
    ///
    /// # Args
    /// * 'col' - the colon to push
    ///
    /// Warning: it only do this upward
    fn push_v(&mut self, col: usize) {
        let mut temp = self.get_col(3, col);
        temp = self.fill_col(3, col, temp);
        temp = self.fill_col(0, col, temp);
        temp = self.fill_col(1, col, temp);
        temp.reverse();
        temp = self.fill_col(5, self.size - col - 1, temp);
        temp.reverse();
        let _ = self.fill_col(3, col, temp);
    }

    /// This function push a row of a cube horizontaly (without rotatning faces if needed)
    /// So this is not a complete move
    ///
    /// # Args
    /// * 'row' - the row to push
    ///
    /// Warning: it only do this to the right
    fn push_h(&mut self, row: usize) {
        let mut temp = self.get_row(2, row);
        for f in [0, 4, 5] {
            temp = self.fill_row(f, row, temp);
        }
        let _ = self.fill_row(2, row, temp);
    }

    /// This function push a row of a cube lateraly (without rotatning faces if needed)
    /// So this is not a complete move
    /// The row pushed belong to the up face (and those which are linked)
    /// So it's not the same that push_h
    ///
    /// # Args
    /// * 'row' - the row to push
    ///
    /// Warning: it only do this to the right
    fn push_l(&mut self, row: usize) {
        let mut temp = self.get_row(1, row);
        temp = self.fill_col(4, self.size - row - 1, temp);
        temp.reverse();
        temp = self.fill_row(3, self.size - row - 1, temp);
        temp = self.fill_col(2, row, temp);
        temp.reverse();
        let _ = self.fill_row(1, row, temp);
    }

    /// This function rotate a face to the left
    /// It only rotate this face so it's not a complete move
    ///
    /// # Args
    /// * 'face' - The face to rotate
    fn rotate_left(&mut self, face: usize) {
        let mut res = Vec::new();
        let mut col;
        for i in 0..self.size {
            col = self.get_col(face, self.size - i - 1);
            for y in col.iter() {
                res.push(*y);
            }
        }
        self.faces[face] = res;
    }

    /// This function rotate a face to the right
    /// It only rotate this face so it's not a complete move
    ///
    /// # Args
    /// * 'face' - The face to rotate
    fn rotate_right(&mut self, face: usize) {
        for _ in 0..3 {
            self.rotate_left(face);
        }
    }

    /// This function apply a downward vertical move to a slice of the cube
    ///
    /// # Args
    /// * 'col' - the vertical slice to move
    pub fn move_vp(&mut self, col: usize) {
        for _ in 0..3 {
            self.move_v(col);
        }
    }

    /// This function apply a upward vertical move to a slice of the cube
    ///
    /// # Args
    /// * 'col' - the vertical slice to move
    pub fn move_v(&mut self, col: usize) {
        self.push_v(col);
        if col == 0 {
            self.rotate_left(2);
        } else if col == self.size - 1 {
            self.rotate_right(4);
        }
    }

    /// This function apply a horizontal move to the left to a slice of the cube
    ///
    /// # Args
    /// * 'row' - the horizontal slice to move
    pub fn move_hp(&mut self, row: usize) {
        for _ in 0..3 {
            self.move_h(row);
        }
    }

    /// This function apply a horizontal move to the right to a slice of the cube
    ///
    /// # Args
    /// * 'row' - the horizontal slice to move
    pub fn move_h(&mut self, row: usize) {
        self.push_h(row);
        if row == 0 {
            self.rotate_left(1);
        } else if row == self.size - 1 {
            self.rotate_right(3);
        }
    }

    /// This function apply a lateral move to the left to a slice of the cube
    ///
    /// # Args
    /// * 'row' - the lateral slice to move
    pub fn move_lp(&mut self, row: usize) {
        for _ in 0..3 {
            self.move_l(row);
        }
    }

    /// This function apply a lateral move to the right to a slice of the cube
    ///
    /// # Args
    /// * 'row' - the lateral slice to move
    pub fn move_l(&mut self, row: usize) {
        self.push_l(row);
        if row == 0 {
            self.rotate_left(5);
        } else if row == self.size - 1 {
            self.rotate_right(0);
        }
    }

    /// This function return true if a face is composed of a unique color,
    /// false otherwise
    ///
    /// # Args
    /// * 'face' - the face as a reference of a color vector
    fn is_face_solved(&self, face: &Vec<Color>) -> bool {
        let color = face[0];

        for c in face.iter() {
            if color != *c {
                return false;
            }
        }
        true
    }

    /// This function return true if cube is completed (all faces are uniform),
    /// false otherwise
    pub fn is_solved(&self) -> bool {
        for f in self.faces.iter() {
            if !self.is_face_solved(f) {
                return false;
            }
        }
        true
    }

    /// This function shuffle the cube by doing some random moves
    /// 
    /// # Args
    /// * 'moves' - the number of move to do
    pub fn shuffle(&mut self, moves: u32) {
        let mut rng = rand::thread_rng();

        for _ in 0..moves {
            let r: u32 = rng.gen_range(1..=6);
            let n: usize = rng.gen_range(0..self.size);

            match r {
                1 => self.move_h(n),
                2 => self.move_hp(n),
                3 => self.move_l(n),
                4 => self.move_lp(n),
                5 => self.move_v(n),
                _ => self.move_vp(n),
            }
        }
    }
}

/// This implementation allow to use our cube struct with prinlnt, print, write...
///
/// # Exemple
/// println("{cube}", cube) -> WWBWWBRRBBBOBBYBBYRRRRRRYYYGGWGGWGGROOWOOWOOWGYYGYYGOO
impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (_, face) in self.faces.iter().enumerate() {
            for c in face {
                write!(f, "{}", c)?;
            }
        }
        Ok(())
    }
}
