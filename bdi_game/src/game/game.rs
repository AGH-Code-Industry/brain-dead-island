use std::time::{Duration, Instant};

use crate::{
    display::sdl::*,
    display::traits::DisplayBuilder,
    input::Input,
    simulation::{simulation::Simulation, world_state::WorldState},
};

use super::logging::init_logging;

pub struct Game {
    display: DisplaySDL,
    simulation: Simulation,
    input: Input,
    sdl: sdl2::Sdl,
    last: Instant,
}

impl Game {
    pub fn init() -> Self {
        init_logging();
        let mut sdl = sdl2::init().unwrap();
        let mut display = DisplayBuilderSDL::new(&mut sdl)
            .set_display("brain dead island", 500, 500)
            .build();

        Game {
            display,
            sdl,
            simulation: Simulation {
                state: WorldState {},
            },
            input: Input {},
            last: Instant::now(),
        }
    }

    pub fn run(&mut self) {
        let fps = 60.0;
        let frame_time = Duration::from_millis((1000.0 / fps) as u64);
        loop {
            let now = Instant::now();
            let diff = now - self.last;

            if diff >= frame_time {
                self.simulation.tick();
                //self.display.render(&self.simulation.state);
            }
            self.last = Instant::now();
        }
    }
}
