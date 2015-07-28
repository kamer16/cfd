use grid;

pub trait Simulation {
    fn get_grid(&mut self) -> &grid::Grid;
    fn update(&mut self, nb_th: u32);
}
