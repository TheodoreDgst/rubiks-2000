#[derive(Clone, Copy, Debug)]
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
        //let mut code = "\x1b[31m";
        let mut code = match self {
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

#[derive(Clone, Debug)]
pub struct Cube {
    pub size: usize,
    pub faces: Vec<Vec<Color>>,
}

impl Cube {
    pub fn new(_size: usize) -> Self {
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

    pub fn get_col(&self, face: usize, col: usize) -> Vec<&Color> {
        let mut res = Vec::new();

        for (index, case) in self.faces[face].iter().enumerate() {
            if index % self.size == col {
                res.push(case);
            }
        }

        res
    }

    pub fn get_row(&self, face: usize, row: usize) -> Vec<&Color> {
        let mut res = Vec::new();
        let stop = self.size * (row + 1);
        for (index, case) in self.faces[face].iter().enumerate().skip(self.size * row) {
            if index < stop {
                res.push(case);
            }
        }
        res
    }
}
