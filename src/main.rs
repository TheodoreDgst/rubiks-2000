mod cube;
mod graphic_cube;
use cube::cube::Cube;
use graphic_cube::gcube::{Gcube, Move::*};

extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::{ light::Light, window::Window};

fn main() {
    let mut window = Window::new("Rubik's 2000");
    window.set_light(Light::StickToCamera);
    let cube = Cube::new_filled(3);

    let mut gcube = Gcube::new(cube, &mut window);

    while window.render() {
        gcube.apply_move(&mut window, H, 0);
        gcube.apply_move(&mut window, Vp, 1);
        gcube.apply_move(&mut window, L, 2);
        gcube.apply_move(&mut window, Lp, 0);
        gcube.apply_move(&mut window, V, 1);
        gcube.apply_move(&mut window, Hp, 3);
    }
}
