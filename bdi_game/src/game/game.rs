use std::time::{Duration, Instant};

use crate::{
    display::game_display::GameDisplay,
    input::Input,
    simulation::{simulation::Simulation, world_state::WorldState},
};

use super::logging::init_logging;

pub struct Game<D: GameDisplay> {
    display: D,
    simulation: Simulation,
    input: Input,
}

impl<D: GameDisplay> Game<D> {
    pub fn init(display: D) -> Game<D> {
        init_logging();

        Game {
            display,
            simulation: Simulation {
                state: WorldState {},
            },
            input: Input {},
        }
    }

    pub fn run(&mut self) {
        let fps = 60.0;
        let frame_time = Duration::from_millis((1000.0 / fps) as u64);
        loop {
            let start = Instant::now();

            self.simulation.tick();
            self.display.render(&self.simulation.state);

            let end = Instant::now();
            let diff = end - start;
            if diff < frame_time {
                ::std::thread::sleep(frame_time - diff);
            }
        }
    }
}
