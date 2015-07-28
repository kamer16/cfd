use std::sync::Arc;
use std::thread;
use std::mem;
use physics::Simulation;
use grid;
use sdl2::rect;

pub struct LinearConvection {
    pub fluid: Arc<grid::Grid>,
    pub buff: grid::Grid,
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
            fluid: Arc::new(grid::Grid::new(width, height)),
            buff: grid::Grid::new(width, height),
            width: width,
            height: height,
            dx: 1. / width as f32,
            dy: 1. / height as f32,
            c: 2., // Velocity: WARNING High velocity generates big errors
            sigma: 0.2,
            dt: 0.2 * 1. / width as f32,
        };
        unsafe {
            res.fluid.make_unique().init(rect::Rect::new_unwrap((width / 4) as i32,
            (height / 4) as i32,
            width / 2, height / 2));
        }
        res
    }
}

impl Simulation for LinearConvection {

    fn get_grid(&mut self) -> &grid::Grid {
        unsafe {
            self.fluid.make_unique()
        }
    }

    fn update(&mut self, nb_th: u32) {
        let chunk_size = (self.height / nb_th * self.width) as usize;
        {
            let mut vec = Vec::new();
            let dt = self.dt;
            let dx = self.dx;
            let dy = self.dy;
            let vel = self.c;
            let width = self.width;
            for (chunk_id, chunk) in self.buff.chunks_mut(chunk_size).enumerate() {
                let data = self.fluid.clone();
                vec.push(thread::scoped(move || {
                    for (id, elt) in chunk.iter_mut().enumerate() {
                        let idx = chunk_id * chunk_size + id;
                        if idx < width as usize {
                            continue;
                        }
                        let c = data[idx];
                        let l = data[(idx - 1)];
                        let t = data[(idx - width as usize)];
                        let mut val = c -
                            (vel * dt / dx * (c - l)) -
                            (vel * dt / dy * (c - t));


                        // Some hardware are slow on subnormal numbers
                        if !val.is_normal() && val != 0.{
                            val = 0.;
                        }
                        *elt = val;
                    }
                }));
            }
        }

        unsafe {
            mem::swap(self.fluid.make_unique(), &mut self.buff);
        }
    }
}
