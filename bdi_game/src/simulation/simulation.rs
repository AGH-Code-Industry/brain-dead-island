use super::world_state::WorldState;
use super::world_grid::WorldGrid;

pub struct Simulation {
    pub state: WorldState,
    pub grid: WorldGrid,
}

impl Simulation {
    pub fn tick(&mut self) {
        todo!()
    }
}
