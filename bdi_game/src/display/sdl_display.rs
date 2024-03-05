use crate::simulation::world_state::WorldState;

use super::game_display::GameDisplay;

// TODO: Implement SDL display

struct SdlDisplay {}

impl GameDisplay for SdlDisplay {
    fn render(&mut self, state: &WorldState) {
        todo!()
    }
}
