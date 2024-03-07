#[cfg(test)]
mod tests {
    use crate::cube::{ cube::Cube, defs::*, enums::Move };
    use rand::{ thread_rng, Rng };

    #[test]
    /// Apply the basics move to a solved cube and check the result
    fn test_multiply() {

        for (index, mv) in ALL_MOVES.iter().enumerate() {
            let mut cube = DEFAULT;
            cube.multiply(*mv);
            assert_eq!(cube, *mv, " Failed move - {}", Move::from(index).to_string());
        }

        for (index, mv) in ALL_MOVES.iter().enumerate() {
            let mut cube = DEFAULT;
            for _ in 0..4 {
                cube.multiply(*mv);
            }
            assert_eq!(cube, DEFAULT, " Failed move - {} * 4", Move::from(index).to_string());
        }

        assert_ne!(MOVE_B, DEFAULT, "Failed to compare, MOVE_B != DEFAULT");

        let mut cube = DEFAULT;
        for _ in 0..6 {
           cube.multiply(MOVE_R);
           cube.multiply(MOVE_U);
           cube.multiply(ALL_MOVES[Move::R3 as usize]);
           cube.multiply(ALL_MOVES[Move::U3 as usize]);
        }
        assert_eq!(cube, DEFAULT);
    }

    #[test]
    fn test_get_twist() {
        let c = DEFAULT;
        assert_eq!(c.get_twist(), 0);

        let c = Cube::new(CP_DEFAULT, [0, 0, 0, 0, 0, 0, 0, 1], EP_DEFAULT, EO_DEFAULT);
        assert_eq!(c.get_twist(), 0);

        let c = Cube::new(CP_DEFAULT, [1, 0, 0, 0, 0, 0, 0, 0], EP_DEFAULT, EO_DEFAULT);
        assert_eq!(c.get_twist(), 729);

        let c = Cube::new(CP_DEFAULT, [2, 2, 2, 2, 2, 2, 2, 1], EP_DEFAULT, EO_DEFAULT);
        assert_eq!(c.get_twist(), 2186);
    }

    #[test]
    fn test_get_flip() {
        let c = DEFAULT;
        assert_eq!(c.get_flip(), 0);

        let c = Cube::new(CP_DEFAULT, CO_DEFAULT, EP_DEFAULT, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
        assert_eq!(c.get_flip(), 0);

        let c = Cube::new(CP_DEFAULT, CO_DEFAULT, EP_DEFAULT, [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
        assert_eq!(c.get_flip(), 1024);

        let c = Cube::new(CP_DEFAULT, CO_DEFAULT, EP_DEFAULT, [1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1]);
        assert_eq!(c.get_flip(), 1124);
    }

    #[test]
    fn test_set_twist() {
        let mut rng = thread_rng();

        let twist = 1;
        let mut c = DEFAULT;
        c.set_twist(twist);

        assert_eq!(c.get_twist(), 1);

        for _ in 0..10 {
            let twist = rng.gen_range(0..N_CORNER_ORI);
            c.set_twist(twist);
            assert_eq!(c.get_twist(), twist);
        }
    }

    #[test]
    fn test_set_flip() {
        let mut rng = thread_rng();

        let flip = 1;
        let mut c = DEFAULT;
        c.set_flip(flip);

        assert_eq!(c.get_flip(), 1);

        for _ in 0..10 {
            let flip = rng.gen_range(0..N_EDGE_PERM);
            c.set_flip(flip);
            assert_eq!(c.get_flip(), flip);
        }
    }

    #[test]
    fn test_convertion_cube_facelet_cube() {
        let cube = DEFAULT;
        let face_cube = cube.to_facelet_cube();
        let final_cube = face_cube.to_cubie_cube();
        assert_eq!(cube, final_cube);
    }
    #[test]
    fn test_move_inv(){
        assert_eq!(Move::F3.move_inv(),Move::F1);
        assert_eq!(Move::F2.move_inv(),Move::F2);
        assert_eq!(Move::F1.move_inv(),Move::F3);
        assert_eq!(Move::U1.move_inv(),Move::U3);
        assert_eq!(Move::U3.move_inv(),Move::U1);
        assert_eq!(Move::U2.move_inv(),Move::U2);
        assert_eq!(Move::D1.move_inv(),Move::D3);
        assert_eq!(Move::D2.move_inv(),Move::D2);
        assert_eq!(Move::D3.move_inv(),Move::D1);
        assert_eq!(Move::B1.move_inv(),Move::B3);
        assert_eq!(Move::B2.move_inv(),Move::B2);
        assert_eq!(Move::B3.move_inv(),Move::B1);
        assert_eq!(Move::L3.move_inv(),Move::L1);
        assert_eq!(Move::L2.move_inv(),Move::L2);
        assert_eq!(Move::L1.move_inv(),Move::L3);
        assert_eq!(Move::R1.move_inv(),Move::R3);
        assert_eq!(Move::R2.move_inv(),Move::R2);
        assert_eq!(Move::R3.move_inv(),Move::R1);
    }
}
