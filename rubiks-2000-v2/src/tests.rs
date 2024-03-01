#[cfg(test)]
mod tests {
    use crate::{
        cube::{cube::Cube, defs::*},
        TablePhase1,
    };
    use rand::{thread_rng, Rng};

    #[test]
    /// Apply the basics move to a solved cube and check the result
    fn test_multiply_basic_moves() {
        // Mouvement "Up"
        let mut c_up = Cube::new_default();
        c_up.multiply(MOVE_U);
        assert_eq!(c_up, MOVE_U);

        // Mouvement "Right"
        let mut c_right = Cube::new_default();
        c_right.multiply(MOVE_R);
        assert_eq!(c_right, MOVE_R);

        // Mouvement "Front"
        let mut c_front = Cube::new_default();
        c_front.multiply(MOVE_F);
        assert_eq!(c_front, MOVE_F);

        // Mouvement "Down"
        let mut c_down = Cube::new_default();
        c_down.multiply(MOVE_D);
        assert_eq!(c_down, MOVE_D);

        // Mouvement "Left"
        let mut c_left = Cube::new_default();
        c_left.multiply(MOVE_L);
        assert_eq!(c_left, MOVE_L);

        // Mouvement "Back"
        let mut c_back = Cube::new_default();
        let back_move = MOVE_B;
        c_back.multiply(back_move);
        assert_eq!(c_back, MOVE_B);
    }

    #[test]
    fn test_tricky_moves() {
        //let mut cube = MOVE_R;

        //cube
    }

    #[test]
    fn test_get_twist() {
        let c = Cube::new_default();
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
        let c = Cube::new_default();
        assert_eq!(c.get_flip(), 0);

        let c = Cube::new(
            CP_DEFAULT,
            CO_DEFAULT,
            EP_DEFAULT,
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        );
        assert_eq!(c.get_flip(), 0);

        let c = Cube::new(
            CP_DEFAULT,
            CO_DEFAULT,
            EP_DEFAULT,
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        );
        assert_eq!(c.get_flip(), 1024);

        let c = Cube::new(
            CP_DEFAULT,
            CO_DEFAULT,
            EP_DEFAULT,
            [1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1],
        );
        assert_eq!(c.get_flip(), 1124);
    }

    #[test]
    fn test_set_twist() {
        let mut rng = thread_rng();

        let twist = 1;
        let mut c = Cube::new_default();
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
        let mut c = Cube::new_default();
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
        let cube = Cube::new_default();
        let face_cube = cube.to_facelet_cube();
        let final_cube = face_cube.to_cubie_cube();
        assert_eq!(cube, final_cube);
    }

    #[test]
    fn test_find_solution_phase1() {
        let bad_eo: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        let c = Cube::new(CP_DEFAULT, CO_DEFAULT, EP_DEFAULT, bad_eo);

        let t = TablePhase1::new(String::from("taable"));

        let mut passed = false;

        match t.find_solution_to_g1(c.get_flip()) {
            None => passed = false,
            _ => passed = true,
        }

        assert_eq!(passed, false);
    }
    // TODO: écrire les tests pour ALL_MOVES (pas sur de l'initialisation)
}
