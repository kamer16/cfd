use sdl2::rect;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Grid {
    nx: u32,
    ny: u32,
    data: Vec<f32>,
}

impl Grid {
    pub fn new(nx: u32, ny: u32) -> Grid {
        Grid {
            nx: nx,
            ny: ny,
            data: vec!(0.; (nx * ny) as usize)
        }
    }

    pub fn init(&mut self, rect: rect::Rect) {
        let (x, y, w, h) = rect.xywh();
        for j in y as u32..y as u32 + h {
            let start_range: usize = (j * self.nx + x as u32) as usize;
            let end_range: usize = start_range + w as usize;
            for idx in start_range..end_range + 1 {
                self.data[idx] = 127.;
            }
        }
    }

    pub fn get_nx(&self) -> u32 { self.nx }
    pub fn get_ny(&self) -> u32 { self.ny }
}

impl Deref for Grid {
    type Target = [f32];

    fn deref(&self) -> &[f32] {
        &self.data[..]
    }
}

impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut [f32] {
        &mut self.data[..]
    }
}
