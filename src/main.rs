extern crate sdl2;

mod physics;
mod grid;
use sdl2::keyboard::Keycode;

use sdl2::pixels::PixelFormatEnum;

fn main() {
    let w = 800;
    let h = 800;
    let mut sys = physics::System::new(w, h);
    let mut sdl_context = sdl2::init().video().unwrap();
    let window = sdl_context.window("Computational fluid dynamics", w, h)
        .position_centered()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    let mut tex = renderer.create_texture_streaming(PixelFormatEnum::RGBA8888,
                                          (w, h))
        .unwrap();
    tex.update(None, sys.get_pixels(), (w * 4) as usize).unwrap();
    renderer.clear();
    renderer.copy(&tex, None, None);
    renderer.present();

    let mut running = true;

    while running {
        for event in sdl_context.event_pump().poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false
                },
                _ => {}
            }
        }
        sys.update();
        tex.update(None, sys.get_pixels(), (w * 4) as usize).unwrap();
        renderer.clear();
        renderer.copy(&tex, None, None);
        renderer.present();
    }
}
