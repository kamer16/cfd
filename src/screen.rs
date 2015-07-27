use sdl2;
use sdl2::pixels::PixelFormatEnum;

use grid;

pub struct Screen<'a> {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
    tex: sdl2::render::Texture,
    renderer: sdl2::render::Renderer<'a>,
}

impl<'a> Screen<'a> {
    pub fn set_title(&mut self, sdl_context: &sdl2::Sdl, fps: u32) {
        if fps == 0 {
            return;
        }
        self.renderer.window_properties(sdl_context).unwrap()
            .set_title(&format!("Computational fluid dynamics: FPS {}", fps));
    }

    pub fn new(w: u32, h: u32, sdl_context: &sdl2::Sdl) -> Screen<'a> {
    let window = sdl_context.window("Computational fluid dynamics", w, h)
        .position_centered()
        .build()
        .unwrap();
        let renderer = window.renderer().build().unwrap();
        Screen {
            width: w,
            height: h,
            pixels: vec!(0; (w * h * 4) as usize),
            tex: renderer.create_texture_streaming(PixelFormatEnum::RGBA8888,
                                                   (w, h)).unwrap(),
            renderer: renderer
        }
    }

    pub fn render(&mut self, grid: &grid::Grid) {
        self.update_pixels(grid);
        self.tex.update(None, &self.pixels, (self.width * 4) as usize).unwrap();
        self.renderer.clear();
        self.renderer.copy(&self.tex, None, None);
        self.renderer.present();
    }

    fn update_pixels(&mut self, grid: &grid::Grid) {
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                let idx = x + y * self.width as usize;
                let val = self.get_average(x as u32, y as u32, grid);
                self.pixels[idx * 4 + 0] = 0;
                self.pixels[idx * 4 + 1] = val;
                self.pixels[idx * 4 + 2] = val;
                self.pixels[idx * 4 + 3] = val;
            }
        }
    }

    fn get_average(&self, x: u32, y: u32, grid: &grid::Grid) -> u8 {
        let stepx = grid.get_nx() / self.width;
        let stepy = grid.get_ny() / self.height;
        let y0 = y * stepy;
        let y1 = (y + 1) * stepy;
        let x0 = x * stepx;
        let x1 = (x + 1) * stepx;

        let mut val = 0.;

        for y_ in y0..y1 {
            for x_ in x0..x1 {
                val += (grid[(x_ + y_ * grid.get_nx()) as usize]) / (stepx * stepy) as f32;
            }
        }

        return val.round() as u8;
    }
}
