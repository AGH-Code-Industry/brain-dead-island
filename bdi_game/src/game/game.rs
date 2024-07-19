use crate::display::camera::Camera;
use crate::display::traits::GameDisplay;
use crate::input::Input;
use crate::simulation::{simulation::Simulation, world_grid::WorldGrid, world_state::WorldState};
use crate::terrain_manager::map_loader::MapLoader;

use super::logging::init_logging;
use crate::util::vec2::Vec2;
use std::time::{Duration, Instant};

pub struct Game<D: GameDisplay> {
    display: D,
    simulation: Simulation,
    input: Input,
}

impl<D: GameDisplay> Game<D> {
    pub fn init(display: D) -> Game<D> {
        init_logging();

        let side_len = 40;
        let map = MapLoader::map_from_image("perlin");

        Game {
            display,
            simulation: Simulation {
                state: WorldState {
                    world_grid: WorldGrid::from_height_map(map, side_len),
                },
            },
            input: Input {},
        }
    }

    pub fn run(&mut self) {
        let fps = 60.0;
        let frame_time = Duration::from_millis((1000.0 / fps) as u64);
        let mut camera = Camera::new(16.0 / 9.0, 10.0);
        camera.set_position(Vec2::new(50.0, 50.0));

        loop {
            let start = Instant::now();

            self.simulation.tick();
            self.display.render(&self.simulation.state, &camera);

            let end = Instant::now();
            let diff = end - start;
            if diff < frame_time {
                ::std::thread::sleep(frame_time - diff);
            }
        }
    }
}
