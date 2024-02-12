extern crate kiss3d;
use kiss3d::light::Light;
use kiss3d::window::Window;

mod graphic_cube;
use graphic_cube::gcube::{self, Move::*};

mod cube;

use cube::cube::Cube;

fn main() {
    let mut window = Window::new("Rubik's 2000");

    let c = Cube::new_filled(3);

    let mut a = gcube::Gcube::new(c, &mut window);    
    

    a.shuffle(&mut window, 1000);


    while window.render() {}
}
