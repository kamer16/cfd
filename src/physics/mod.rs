use sdl2::rect;
use grid;

pub struct System {
    pub fluid: grid::Grid,
}

impl System {
    pub fn new(width: u32, height: u32) -> System{
        let mut res = System {
            fluid: grid::Grid::new(width, height),
        };
        res.fluid.init(rect::Rect::new_unwrap((width / 4) as i32,
                                              (height / 4) as i32,
                                              width / 2, height / 2));
        res
    }
}
