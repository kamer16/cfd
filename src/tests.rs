#[cfg(test)]

use test::Bencher;
use sdl2;
use screen;
use physics;
pub use physics::Simulation;

#[bench]
fn bench_update1(b: &mut Bencher) {
    let (w, h) = (800, 800);
    let (nx, ny) = (w * 1, h * 1);
    let mut sys = physics::LinearConvection::new(nx, ny);

    b.iter(|| sys.update(1));
}

#[bench]
fn bench_update2(b: &mut Bencher) {
    let (w, h) = (800, 800);
    let (nx, ny) = (w * 1, h * 1);
    let mut sys = physics::LinearConvection::new(nx, ny);

    b.iter(|| sys.update(2));
}

#[bench]
fn bench_update8(b: &mut Bencher) {
    let (w, h) = (800, 800);
    let (nx, ny) = (w * 1, h * 1);
    let mut sys = physics::LinearConvection::new(nx, ny);

    b.iter(|| sys.update(8));
}

#[bench]
fn bench_update14(b: &mut Bencher) {
    let (w, h) = (800, 800);
    let (nx, ny) = (w * 1, h * 1);
    let mut sys = physics::LinearConvection::new(nx, ny);

    b.iter(|| sys.update(14));
}

#[bench]
fn bench_update20(b: &mut Bencher) {
    let (w, h) = (800, 800);
    let (nx, ny) = (w * 1, h * 1);
    let mut sys = physics::LinearConvection::new(nx, ny);

    b.iter(|| sys.update(20));
}

#[bench]
fn bench_render1(b: &mut Bencher) {
    let (w, h) = (800, 800);
    let (nx, ny) = (w * 1, h * 1);
    let mut sdl_context = sdl2::init().video().unwrap();
    let mut screen = screen::Screen::new(w, h, &sdl_context);
    let mut sys = physics::LinearConvection::new(nx, ny);
    let grid = sys.get_grid();
    b.iter(|| screen.render(grid));
}

#[bench]
fn bench_cavity_flow1(b: &mut Bencher) {
    let (w, h) = (800, 800);
    let (nx, ny) = (w * 1, h * 1);
    let mut sys = physics::CavityFlow::new(nx, ny);

    b.iter(|| sys.update(1));
}

#[bench]
fn bench_cavity_flow2(b: &mut Bencher) {
    let (w, h) = (800, 800);
    let (nx, ny) = (w * 1, h * 1);
    let mut sys = physics::CavityFlow::new(nx, ny);

    b.iter(|| sys.update(2));
}

#[bench]
fn bench_cavity_flow8(b: &mut Bencher) {
    let (w, h) = (800, 800);
    let (nx, ny) = (w * 1, h * 1);
    let mut sys = physics::CavityFlow::new(nx, ny);

    b.iter(|| sys.update(8));
}

#[bench]
fn bench_cavity_flow14(b: &mut Bencher) {
    let (w, h) = (800, 800);
    let (nx, ny) = (w * 1, h * 1);
    let mut sys = physics::CavityFlow::new(nx, ny);

    b.iter(|| sys.update(14));
}

#[bench]
fn bench_cavity_flow20(b: &mut Bencher) {
    let (w, h) = (800, 800);
    let (nx, ny) = (w * 1, h * 1);
    let mut sys = physics::CavityFlow::new(nx, ny);

    b.iter(|| sys.update(20));
}
