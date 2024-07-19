use crate::display::camera::Camera;
use crate::display::renderable_objects::grid_hexagon::GridHexagon;
use crate::display::sdl::{RendererBuilderSDL, RendererSDL};
use crate::display::traits::{GameDisplay, RenderableObject, Renderer};
use crate::simulation::grid::GridPoint;
use crate::simulation::world_state::WorldState;
use crate::util::vec2::Vec2;
use sdl2::pixels::Color;

pub struct NullDisplay {}
pub struct NullRenderer {}

impl Renderer for NullRenderer {
    fn render_polygon(&mut self, vertices: &Vec<Vec2>, color: Color) {
        ()
    }

    fn render_line(&mut self, start: Vec2, end: Vec2, color: Color) {
        ()
    }

    fn render_tex(&mut self, pos: Vec2, tex: &str) {
        ()
    }

    fn present(&mut self) {
        ()
    }
}

impl GameDisplay for NullDisplay {
    fn render(&mut self, state: &WorldState, camera: &Camera) {
        ()
    }
}

pub struct SdlDisplay {
    renderer: RendererSDL,
}

impl SdlDisplay {
    pub fn new(name: &str, width: u32, height: u32) -> SdlDisplay {
        let renderer = RendererBuilderSDL::new()
            .set_display(name, width, height)
            .build();

        SdlDisplay { renderer }
    }
}

impl GameDisplay for SdlDisplay {
    fn render(&mut self, state: &WorldState, camera: &Camera) {
        let mut offset: i32 = 0;
        let mut hexagon_vertices: Vec<GridHexagon> = Vec::new();

        for r in 0..2 * state.world_grid.get_side_len() as i32 {
            for q in -offset..state.world_grid.get_side_len() as i32 - offset {
                let point = GridPoint::new(q, r);
                let cell_data = state.world_grid.get_cell(&point);
                let size = 25.0;

                let offset_q_x = q as f32;
                let offset_q_y = 0f32;

                let offset_r_x = 0.5 * r as f32;
                let offset_r_y = 3.0f32.sqrt() / 2.0 * r as f32;

                let x = (offset_q_x + offset_r_x) * 1.5;
                let y = (offset_q_y + offset_r_y) / 2.0;

                hexagon_vertices.push(GridHexagon {
                    center: Vec2::new(x, y) * size,
                    elevation: cell_data.data.elevation,
                    size: size / 2.0,
                });
            }

            if r % 2 == 0 {
                offset += 1;
            }
        }

        for hexagon in hexagon_vertices {
            hexagon.render(camera, &mut self.renderer);
        }

        self.renderer.present();
    }
}
