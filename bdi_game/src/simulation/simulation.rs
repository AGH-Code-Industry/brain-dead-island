use super::world_grid::WorldGrid;
use super::world_state::WorldState;

pub struct Simulation {
    pub state: WorldState,
    pub grid: WorldGrid,
}

impl Simulation {
    pub fn tick(&mut self) {
        todo!()
    }
}
