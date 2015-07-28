#[cfg(test)]

use test::Bencher;
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
fn bench_update3(b: &mut Bencher) {
    let (w, h) = (800, 800);
    let (nx, ny) = (w * 1, h * 1);
    let mut sys = physics::LinearConvection::new(nx, ny);

    b.iter(|| sys.update(8));
}

#[bench]
fn bench_update4(b: &mut Bencher) {
    let (w, h) = (800, 800);
    let (nx, ny) = (w * 1, h * 1);
    let mut sys = physics::LinearConvection::new(nx, ny);

    b.iter(|| sys.update(14));
}

#[bench]
fn bench_update5(b: &mut Bencher) {
    let (w, h) = (800, 800);
    let (nx, ny) = (w * 1, h * 1);
    let mut sys = physics::LinearConvection::new(nx, ny);

    b.iter(|| sys.update(20));
}
