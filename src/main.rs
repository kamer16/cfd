extern crate sdl2;
extern crate time;

mod physics;
mod screen;
mod grid;
mod fps;

use sdl2::keyboard::Keycode;
pub use physics::Simulation;

fn main() {
    let (w, h) = (800, 800);
    let (nx, ny) = (w * 1, h * 1);
    let mut sys = physics::LinearConvection::new(nx, ny);
    let mut sdl_context = sdl2::init().video().unwrap();
    let mut screen = screen::Screen::new(w, h, &sdl_context);

    let mut running = true;
    let mut fps = fps::Fps::new();

    while running {
        screen.render(sys.get_grid());
        sys.update();

        for event in sdl_context.event_pump().poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false
                },
                _ => {}
            }
        }
        screen.set_title(&sdl_context, fps.update());
    }
}
