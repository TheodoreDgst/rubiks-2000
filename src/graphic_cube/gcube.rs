use std::{
    borrow::BorrowMut,
    cell::RefCell,
    rc::Rc,
    time::{Duration, Instant},
    usize,
};

use crate::cube::cube::Cube;
use kiss3d::{
    nalgebra::{Isometry3, Translation3, UnitQuaternion},
    scene::SceneNode,
    window::Window,
};
use rand::Rng;

const SIZE: f32 = 1.0;
const OFFSET: f32 = 0.2;
const ANIMATION_TIME: f32 = 0.5;

#[derive(PartialEq)]
pub enum Move {
    V,
    Vp,
    H,
    Hp,
    L,
    Lp,
}

#[derive(Clone)]
struct Cublet {
    node: SceneNode,
    front_face: SceneNode,
    back_face: SceneNode,
    left_face: SceneNode,
    right_face: SceneNode,
    top_face: SceneNode,
    bottom_face: SceneNode,
    position: (usize, usize, usize),
}

impl Cublet {
    pub fn new() -> Self {
        let mut cubelet = SceneNode::new_empty();

        // Create faces
        let mut front_face = cubelet.add_quad(SIZE, SIZE, 1, 1);
        let mut back_face = cubelet.add_quad(SIZE, SIZE, 1, 1);
        let mut right_face = cubelet.add_quad(SIZE, SIZE, 1, 1);
        let mut left_face = cubelet.add_quad(SIZE, SIZE, 1, 1);
        let mut top_face = cubelet.add_quad(SIZE, SIZE, 1, 1);
        let mut bottom_face = cubelet.add_quad(SIZE, SIZE, 1, 1);

        // Set faces positions
        let half_size = SIZE / 2.0;
        back_face.set_local_translation(Translation3::new(0.0, 0.0, half_size));
        front_face.set_local_translation(Translation3::new(0.0, 0.0, -half_size));
        right_face.set_local_translation(Translation3::new(-half_size, 0.0, 0.0));
        left_face.set_local_translation(Translation3::new(half_size, 0.0, 0.0));
        top_face.set_local_translation(Translation3::new(0.0, half_size, 0.0));
        bottom_face.set_local_translation(Translation3::new(0.0, -half_size, 0.0));

        // Set faces color to blac
        back_face.set_color(0.0, 0.0, 0.0);
        front_face.set_color(0.0, 0.0, 0.0);
        left_face.set_color(0.0, 0.0, 0.0);
        right_face.set_color(0.0, 0.0, 0.0);
        top_face.set_color(0.0, 0.0, 0.0);
        bottom_face.set_color(0.0, 0.0, 0.0);

        // Apply face rotation
        back_face.set_local_rotation(UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0));
        front_face.set_local_rotation(UnitQuaternion::from_euler_angles(
            0.0,
            std::f32::consts::PI,
            0.0,
        ));
        right_face.set_local_rotation(UnitQuaternion::from_euler_angles(
            0.0,
            std::f32::consts::FRAC_PI_2,
            0.0,
        ));
        left_face.set_local_rotation(UnitQuaternion::from_euler_angles(
            0.0,
            -std::f32::consts::FRAC_PI_2,
            0.0,
        ));
        top_face.set_local_rotation(UnitQuaternion::from_euler_angles(
            -std::f32::consts::FRAC_PI_2,
            0.0,
            0.0,
        ));
        bottom_face.set_local_rotation(UnitQuaternion::from_euler_angles(
            std::f32::consts::FRAC_PI_2,
            0.0,
            0.0,
        ));

        Cublet {
            node: cubelet,
            front_face: front_face,
            back_face: back_face,
            left_face: left_face,
            right_face: right_face,
            top_face: top_face,
            bottom_face: bottom_face,
            position: (0, 0, 0),
        }
    }

    /// This function set the local position of the cublet (relativly to the cube)
    /// Warning: using this function will not change the real position of the cublet in the 3D space.
    /// The 'position' attribute is only used to now what cublet to move when needed...
    ///
    /// # Args
    /// * 'x, y, z': the new position of the cublet
    pub fn set_position(&mut self, x: usize, y: usize, z: usize) {
        self.position = (x, y, z);
    }
}

pub struct Gcube {
    cube: Cube,
    node: SceneNode,
    cublets: Vec<Cublet>,
}

impl Gcube {
    pub fn new(_cube: Cube, window: &mut Window) -> Self {
        let mut _node = window.add_group();
        let s = (_cube.get_size() as i32) / 2;
        let mut _cublets = Vec::new();
        let pair = (_cube.get_size() as i32) % 2 == 0;

        for i in -s..=s {
            if i == 0 && pair {
                continue;
            }
            let fx = i as f32;
            for j in -s..=s {
                if j == 0 && pair {
                    continue;
                }
                let fy = j as f32;
                for k in -s..=s {
                    if k == 0 && pair {
                        continue;
                    }
                    let fz = k as f32;
                    let mut cublet = Cublet::new();
                    let translation = if !pair {
                        Isometry3::translation(fx * OFFSET + fx, fy * OFFSET + fy, fz * OFFSET + fz)
                    } else {
                        Isometry3::translation(
                            (fx - fx.signum() * 0.5) * OFFSET + (fx - fx.signum() * 0.5),
                            (fy - fy.signum() * 0.5) * OFFSET + (fy - fy.signum() * 0.5),
                            (fz - fz.signum() * 0.5) * OFFSET + (fz - fz.signum() * 0.5),
                        )
                    };
                    cublet.node.set_local_transformation(translation);
                    cublet.set_position(
                        (i + s - (pair && i > 0) as i32) as usize,
                        (j + s - (pair && j > 0) as i32) as usize,
                        (k + s - (pair && k > 0) as i32) as usize,
                    );
                    if k == s {
                        cublet.back_face.set_color(1.0, 1.0, 0.0);
                    } else if k == -s {
                        cublet.front_face.set_color(1.0, 1.0, 1.0)
                    }
                    if i == s {
                        cublet.left_face.set_color(1.0, 0.0, 0.0)
                    } else if i == -s {
                        cublet.right_face.set_color(1.0, 0.5, 0.0)
                    }
                    if j == s {
                        cublet.top_face.set_color(0.0, 0.0, 1.0)
                    } else if j == -s {
                        cublet.bottom_face.set_color(0.0, 1.0, 0.0);
                    }
                    let rc_child = Rc::new(RefCell::new(cublet));
                    let child_node = Rc::clone(&rc_child).borrow().node.clone();
                    let cublet = Rc::clone(&rc_child).borrow().clone();
                    _node.add_child(child_node);
                    _cublets.push(cublet);
                }
            }
        }

        Gcube {
            cube: _cube,
            node: _node,
            cublets: _cublets,
        }
    }

    pub fn shuffle(&mut self, window: &mut Window ,moves: u32) {
        let mut rng = rand::thread_rng();

        for _ in 0..moves {
            let r: u32 = rng.gen_range(1..=6);
            let n: usize = rng.gen_range(0..self.cube.get_size());

            let mov = match r {
                1 => Move::H,
                2 => Move::Hp,
                3 => Move::L,
                4 => Move::Lp,
                5 => Move::V,
                _ => Move::Vp,
            };

            self.apply_move(window, mov, n);
        }
    }

    fn adjust_cublet_positions(&mut self) {
        let precision = 0.1;
        for i in 0..self.cublets.len() {
            let cublet = self.cublets[i].borrow_mut();
            let x = cublet.node.data().local_transformation().translation.x;
            let y = cublet.node.data().local_transformation().translation.y;
            let z = cublet.node.data().local_transformation().translation.z;
            let translation = Translation3::new(
                (x / precision).round() * precision,
                (y / precision).round() * precision,
                (z / precision).round() * precision,
            );
            cublet.node.set_local_translation(translation);
        }
    }

    /// This function make the graphic cube execute move with animation
    /// 
    /// # Args
    /// * 'window' - a reference to the window where the cube is displayed
    /// * 'mov' - the type of move
    /// * 'n' - the stripe concerned by the move (depending on the type)
    /// 
    /// # Warning: 
    /// The time of the animation will depend on the global constant 'ANIMATION_DURATION'
    pub fn apply_move(&mut self, window: &mut Window, mov: Move, n: usize) {
        let animation_duration = Duration::from_secs_f32(ANIMATION_TIME);
        let start_time = Instant::now();
        let mut prev = 0.0;
        let s = self.cube.get_size() - 1;

        // Determine factors (the axe of the rotation)
        let (roll_fact, pitch_fact, yaw_fact) = match mov {
            Move::V => (1.0, 0.0, 0.0),
            Move::Vp => (-1.0, 0.0, 0.0),
            Move::H => (0.0, 1.0, 0.0),
            Move::Hp => (0.0, -1.0, 0.0),
            Move::L => (0.0, 0.0, 1.0),
            Move::Lp => (0.0, 0.0, -1.0),
        };

        // Functions to determine which cublet to move depending of the move type
        let f_v_move = |(x, _, _): (usize, usize, usize)| x;
        let f_h_move = |(_, y, _): (usize, usize, usize)| y;
        let f_l_move = |(_, _, z): (usize, usize, usize)| z;

        // Functions to determine which coordinates to update depending of the move type
        let f_h_coord = |cub: &mut Cublet| {
            let (x, y, z) = cub.position;
            cub.set_position(z, y, s - x);
        };
        let f_l_coord = |cub: &mut Cublet| {
            let (x, y, z) = cub.position;
            cub.set_position(s - y, x, z);
        };
        let f_v_coord = |cub: &mut Cublet| {
            let (x, y, z) = cub.position;
            cub.set_position(x, s - z, y);
        };
        let f_hp_coord = |cub: &mut Cublet| {
            let (x, y, z) = cub.position;
            cub.set_position(s - z, y, x);
        };
        let f_lp_coord = |cub: &mut Cublet| {
            let (x, y, z) = cub.position;
            cub.set_position(y, s - x, z);
        };
        let f_vp_coord = |cub: &mut Cublet| {
            let (x, y, z) = cub.position;
            cub.set_position(x, z, s - y);
        };

        let f_move = if mov == Move::H || mov == Move::Hp {
            f_h_move
        } else if mov == Move::L || mov == Move::Lp {
            f_l_move
        } else {
            f_v_move
        };

        // Animation loop
        while window.render() {
            let elapsed_time = start_time.elapsed();
            let delta = (elapsed_time.as_secs_f32() - prev) / animation_duration.as_secs_f32();
            prev = elapsed_time.as_secs_f32();
            let rotation_angle = delta * std::f32::consts::FRAC_PI_2;
            let rotation_quaternion = UnitQuaternion::from_euler_angles(
                rotation_angle * roll_fact,
                rotation_angle * pitch_fact,
                rotation_angle * yaw_fact,
            );

            // Apply rotation to all cublets concerned (determined by f)
            for i in 0..self.cublets.len() {
                let cublet = self.cublets[i].borrow_mut();
                if f_move(cublet.position) == n {
                    cublet.node.append_rotation(&rotation_quaternion);
                }
            }

            if elapsed_time >= animation_duration {
                self.adjust_cublet_positions();
                break;
            }
        }

        // Update local coordinates
        for i in 0..self.cublets.len() {
            let cublet = self.cublets[i].borrow_mut();
            if f_move(cublet.position) == n {
                if mov == Move::H {
                    f_h_coord(cublet);
                } else if mov == Move::Hp {
                    f_hp_coord(cublet);
                } else if mov == Move::L {
                    f_l_coord(cublet);
                } else if mov == Move::Lp {
                    f_lp_coord(cublet);
                } else if mov == Move::V {
                    f_v_coord(cublet);
                } else {
                    f_vp_coord(cublet);
                }
            }
        }
    }

    /// This function will highlight the xth vertical stripe of the cube
    /// with a random color
    pub fn dbg_position_x(&mut self, x: usize) {
        let mut rng = rand::thread_rng();
        let r: f32 = rng.gen_range(0.0..=1.0);
        let g: f32 = rng.gen_range(0.0..=1.0);
        let b: f32 = rng.gen_range(0.0..=1.0);
        for i in 0..self.cublets.len() {
            let cublet = self.cublets[i].borrow_mut();
            if cublet.position.0 == x {
                cublet.node.set_color(r, g, b);
            }
        }
    }

    /// This function will highlight the yth horizontal stripe of the cube
    /// with a random color
    pub fn dbg_position_y(&mut self, y: usize) {
        let mut rng = rand::thread_rng();
        let r: f32 = rng.gen_range(0.0..=1.0);
        let g: f32 = rng.gen_range(0.0..=1.0);
        let b: f32 = rng.gen_range(0.0..=1.0);
        for i in 0..self.cublets.len() {
            let cublet = self.cublets[i].borrow_mut();
            if cublet.position.1 == y {
                cublet.node.set_color(r, g, b);
            }
        }
    }

    /// This function will highlight the zth lateral stripe of the cube
    /// with a random color
    pub fn dbg_position_z(&mut self, z: usize) {
        let mut rng = rand::thread_rng();
        let r: f32 = rng.gen_range(0.0..=1.0);
        let g: f32 = rng.gen_range(0.0..=1.0);
        let b: f32 = rng.gen_range(0.0..=1.0);
        for i in 0..self.cublets.len() {
            let cublet = self.cublets[i].borrow_mut();
            if cublet.position.2 == z {
                cublet.node.set_color(r, g, b);
            }
        }
    }
}

fn round(n: f32, precision: f32) -> f32 {
    (n / precision).round() * precision
}
