use sdl2::rect;
use grid;

pub struct System {
    pixels: Vec<u8>,
    pub fluid: grid::Grid,
    width: u32,
    height: u32,
    dx: f32,
    dy: f32,
    c: f32, // Velocity
    sigma: f32,
    dt: f32,
}

impl System {
    pub fn new(width: u32, height: u32) -> System{
        let mut res = System {
            pixels: vec!(0; (width * height * 4) as usize),
            fluid: grid::Grid::new(width, height),
            width: width,
            height: height,
            dx: 1. / width as f32,
            dy: 1. / height as f32,
            c: 2., // Velocity: WARNING High velocity generates big errors
            sigma: 0.2,
            dt: 0.2 * 1. / width as f32,
        };
        res.fluid.init(rect::Rect::new_unwrap((width / 4) as i32,
                                              (height / 4) as i32,
                                              width / 2, height / 2));
        res
    }
    pub fn update(&mut self) {
        for y in 1..self.height as usize {
            for x in 1..self.width as usize {
                let u = &mut self.fluid;
                let idx = x + y * self.width as usize;
                let c = u[idx];
                let l = u[(idx - 1)];
                let t = u[(idx - self.width as usize)];
                let val = c -
                    (self.c * self.dt / self.dx * (c - l)) -
                    (self.c * self.dt / self.dy * (c - t));


                u[idx] = val;
                self.pixels[idx * 4 + 0] = 0;
                self.pixels[idx * 4 + 1] = val.round() as u8;
                self.pixels[idx * 4 + 2] = val.round() as u8;
                self.pixels[idx * 4 + 3] = val.round() as u8;
            }
        }
    }

    pub fn get_pixels(& self) -> &[u8] {
        &self.pixels
    }
}
