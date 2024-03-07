use super::cube::Cube;
use super::enums::{ Color as Cl, Corner as Co, Edge as Ed, Facelet as Fc };
use lazy_static::lazy_static; // Used to create ALL_MOVES (because we cannot initialize with a not const functions a const at runtime)

//////////////////////// Some constants about Rubiks' Cube ////////////////////////////////////////////////////////////////
pub const N_CORNER_ORI: u16 = 2187;
pub const N_EDGE_PERM: u16 = 2048;
pub const MAX_DEPTH_PHASE_1: u8 = 7; // I think it's 7 rather
pub const N_COLORS: usize = 6;
pub const N_EDGES: usize = 12;
pub const N_CORNERS: usize = 8;
pub const N_MOVES: usize = 18;
pub const N_BASE_MOVES: usize = 6;

/////////////////////// The default permutations and orientations of a cube ///////////////////////////////////////////////

// Corner permutations
pub const CP_DEFAULT: [Co; 8] = [
    Co::URF,
    Co::UFL,
    Co::ULB,
    Co::UBR,
    Co::DFR,
    Co::DLF,
    Co::DBL,
    Co::DRB,
];

// Corner orientations
pub const CO_DEFAULT: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

// Edge permutations
pub const EP_DEFAULT: [Ed; 12] = [
    Ed::UR,
    Ed::UF,
    Ed::UL,
    Ed::UB,
    Ed::DR,
    Ed::DF,
    Ed::DL,
    Ed::DB,
    Ed::FR,
    Ed::FL,
    Ed::BL,
    Ed::BR,
];

// Edge orientation
pub const EO_DEFAULT: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

////////////////////// The permutations and orientations to create the basic moves ////////////////////////////////////////

pub const CP_U: [Co; 8] = [Co::UBR, Co::URF, Co::UFL, Co::ULB, Co::DFR, Co::DLF, Co::DBL, Co::DRB];
pub const CO_U: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
pub const EP_U: [Ed; 12] = [
    Ed::UB,
    Ed::UR,
    Ed::UF,
    Ed::UL,
    Ed::DR,
    Ed::DF,
    Ed::DL,
    Ed::DB,
    Ed::FR,
    Ed::FL,
    Ed::BL,
    Ed::BR,
];
pub const EO_U: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

pub const CP_R: [Co; 8] = [Co::DFR, Co::UFL, Co::ULB, Co::URF, Co::DRB, Co::DLF, Co::DBL, Co::UBR];
pub const CO_R: [u8; 8] = [2, 0, 0, 1, 1, 0, 0, 2];
pub const EP_R: [Ed; 12] = [
    Ed::FR,
    Ed::UF,
    Ed::UL,
    Ed::UB,
    Ed::BR,
    Ed::DF,
    Ed::DL,
    Ed::DB,
    Ed::DR,
    Ed::FL,
    Ed::BL,
    Ed::UR,
];
pub const EO_R: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

pub const CP_F: [Co; 8] = [Co::UFL, Co::DLF, Co::ULB, Co::UBR, Co::URF, Co::DFR, Co::DBL, Co::DRB];
pub const CO_F: [u8; 8] = [1, 2, 0, 0, 2, 1, 0, 0];
pub const EP_F: [Ed; 12] = [
    Ed::UR,
    Ed::FL,
    Ed::UL,
    Ed::UB,
    Ed::DR,
    Ed::FR,
    Ed::DL,
    Ed::DB,
    Ed::UF,
    Ed::DF,
    Ed::BL,
    Ed::BR,
];
pub const EO_F: [u8; 12] = [0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0];

pub const CP_D: [Co; 8] = [Co::URF, Co::UFL, Co::ULB, Co::UBR, Co::DLF, Co::DBL, Co::DRB, Co::DFR];
pub const CO_D: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
pub const EP_D: [Ed; 12] = [
    Ed::UR,
    Ed::UF,
    Ed::UL,
    Ed::UB,
    Ed::DF,
    Ed::DL,
    Ed::DB,
    Ed::DR,
    Ed::FR,
    Ed::FL,
    Ed::BL,
    Ed::BR,
];
pub const EO_D: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

pub const CP_L: [Co; 8] = [Co::URF, Co::ULB, Co::DBL, Co::UBR, Co::DFR, Co::UFL, Co::DLF, Co::DRB];
pub const CO_L: [u8; 8] = [0, 1, 2, 0, 0, 2, 1, 0];
pub const EP_L: [Ed; 12] = [
    Ed::UR,
    Ed::UF,
    Ed::BL,
    Ed::UB,
    Ed::DR,
    Ed::DF,
    Ed::FL,
    Ed::DB,
    Ed::FR,
    Ed::UL,
    Ed::DL,
    Ed::BR,
];
pub const EO_L: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

pub const CP_B: [Co; 8] = [Co::URF, Co::UFL, Co::UBR, Co::DRB, Co::DFR, Co::DLF, Co::ULB, Co::DBL];
pub const CO_B: [u8; 8] = [0, 0, 1, 2, 0, 0, 2, 1];
pub const EP_B: [Ed; 12] = [
    Ed::UR,
    Ed::UF,
    Ed::UL,
    Ed::BR,
    Ed::DR,
    Ed::DF,
    Ed::DL,
    Ed::BL,
    Ed::FR,
    Ed::FL,
    Ed::UB,
    Ed::DB,
];

pub const EO_B: [u8; 12] = [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1];

////////////////////// The basic move represented with cubes ///////////////////////////////////////////////////////////////

// Identity move = default cube
pub const DEFAULT: Cube = Cube::new(CP_DEFAULT, CO_DEFAULT, EP_DEFAULT, EO_DEFAULT);

// Up move
pub const MOVE_U: Cube = Cube::new(CP_U, CO_U, EP_U, EO_U);

// Right move
pub const MOVE_R: Cube = Cube::new(CP_R, CO_R, EP_R, EO_R);

// Front move
pub const MOVE_F: Cube = Cube::new(CP_F, CO_F, EP_F, EO_F);

// Down move
pub const MOVE_D: Cube = Cube::new(CP_D, CO_D, EP_D, EO_D);

// Left move
pub const MOVE_L: Cube = Cube::new(CP_L, CO_L, EP_L, EO_L);

// Back move
pub const MOVE_B: Cube = Cube::new(CP_B, CO_B, EP_B, EO_B);

// The six basic moves
pub const BASE_MOVES: [Cube; 6] = [MOVE_U, MOVE_R, MOVE_F, MOVE_D, MOVE_L, MOVE_B];

// Initialize the ALL_MOVES static value
fn initialize_all_moves() -> [Cube; 18] {
    let mut all_move_tab = [DEFAULT; 18];

    for i in 0..N_BASE_MOVES {
        let mut cube = DEFAULT;
        for j in 0..3 {
            cube.multiply(BASE_MOVES[i]);
            all_move_tab[3 * i + j] = cube;
        }
    }

    all_move_tab
}

// Use lazy_static macros to initialize ALL_MOVES
// ALL_MOVES is (if we did it well) not modifiable
// ALL_MOVES contains all the six basic moves 1,2 and 3 times (thus len == 18)
lazy_static! {
    pub static ref ALL_MOVES: [Cube; 18] = initialize_all_moves();
}

//Map the corner positions to facelet positions.
pub const CORNER_FACELET: [[Fc; 3]; 8] = [
    [Fc::U9, Fc::R1, Fc::F3],
    [Fc::U7, Fc::F1, Fc::L3],
    [Fc::U1, Fc::L1, Fc::B3],
    [Fc::U3, Fc::B1, Fc::R3],
    [Fc::D3, Fc::F9, Fc::R7],
    [Fc::D1, Fc::L9, Fc::F7],
    [Fc::D7, Fc::B9, Fc::L7],
    [Fc::D9, Fc::R9, Fc::B7],
];

//Map the edge positions to facelet positions.
pub const EDGE_FACELET: [[Fc; 2]; 12] = [
    [Fc::U6, Fc::R2],
    [Fc::U8, Fc::F2],
    [Fc::U4, Fc::L2],
    [Fc::U2, Fc::B2],
    [Fc::D6, Fc::R8],
    [Fc::D2, Fc::F8],
    [Fc::D4, Fc::L8],
    [Fc::D8, Fc::B8],
    [Fc::F6, Fc::R4],
    [Fc::F4, Fc::L6],
    [Fc::B6, Fc::L4],
    [Fc::B4, Fc::R6],
];

// Map the corner positions to facelet colors.
pub const CORNER_COLOR: [[Cl; 3]; 8] = [
    [Cl::U, Cl::R, Cl::F],
    [Cl::U, Cl::F, Cl::L],
    [Cl::U, Cl::L, Cl::B],
    [Cl::U, Cl::B, Cl::R],
    [Cl::D, Cl::F, Cl::R],
    [Cl::D, Cl::L, Cl::F],
    [Cl::D, Cl::B, Cl::L],
    [Cl::D, Cl::R, Cl::B],
];

// Map the edge positions to facelet colors.
pub const EDGE_COLOR: [[Cl; 2]; 12] = [
    [Cl::U, Cl::R],
    [Cl::U, Cl::F],
    [Cl::U, Cl::L],
    [Cl::U, Cl::B],
    [Cl::D, Cl::R],
    [Cl::D, Cl::F],
    [Cl::D, Cl::L],
    [Cl::D, Cl::B],
    [Cl::F, Cl::R],
    [Cl::F, Cl::L],
    [Cl::B, Cl::L],
    [Cl::B, Cl::R],
];
