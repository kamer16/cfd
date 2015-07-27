use physics::Simulation;
use grid;
use sdl2::rect;

pub struct LinearConvection {
    pub fluid: grid::Grid,
    width: u32,
    height: u32,
    dx: f32,
    dy: f32,
    c: f32, // Velocity
    sigma: f32,
    dt: f32,
}

impl LinearConvection {
    pub fn new(width: u32, height: u32) -> LinearConvection {
        let mut res = LinearConvection {
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
}

impl Simulation for LinearConvection {

    fn get_grid(& self) -> &grid::Grid {
        &self.fluid
    }

    fn update(&mut self) {
        for y in 1..self.height as usize {
            for x in 1..self.width as usize {
                let u = &mut self.fluid;
                let idx = x + y * self.width as usize;
                let c = u[idx];
                let l = u[(idx - 1)];
                let t = u[(idx - self.width as usize)];
                let mut val = c -
                    (self.c * self.dt / self.dx * (c - l)) -
                    (self.c * self.dt / self.dy * (c - t));


                // Some hardware are slow on subnormal numbers
                if !val.is_normal() && val != 0.{
                    val = 0.;
                }
                u[idx] = val;
            }
        }
    }
}
