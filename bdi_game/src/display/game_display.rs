use crate::simulation::world_state::WorldState;

pub trait GameDisplay {
    fn render(&mut self, state: &WorldState);
}

pub struct NullDisplay {}

impl GameDisplay for NullDisplay {
    fn render(&mut self, state: &WorldState) {
        ()
    }
}
