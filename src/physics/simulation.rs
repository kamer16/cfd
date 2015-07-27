use grid;

pub trait Simulation {
    fn get_grid(& self) -> &grid::Grid;
    fn update(&mut self);
}
