use sdl2::rect;
use std::ops::Deref;

#[derive(Debug)]
pub struct Grid {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Grid {
        Grid {
            width: width,
            height: height,
            data: vec!(0; (width * height * 4) as usize)
        }
    }

    pub fn init(&mut self, rect: rect::Rect) {
        let (x, y, w, h) = rect.xywh();
        for j in y as u32..y as u32 + h {
            let start_range: usize = (j * self.width + x as u32) as usize;
            let end_range: usize = start_range + w as usize;
            for idx in start_range..end_range + 1 {
                self.data[idx * 4 + 0] = 0;
                self.data[idx * 4 + 1] = 127;
                self.data[idx * 4 + 2] = 127;
                self.data[idx * 4 + 3] = 127;
            }
        }
    }
}

impl Deref for Grid {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.data[..]
    }
}
