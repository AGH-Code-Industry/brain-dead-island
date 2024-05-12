use sdl2::pixels::Color;
use crate::display::camera::Camera;
use crate::display::traits::{GameDisplay, RenderableObject, Renderer, RendererBuilder};
use crate::simulation::grid::GridPoint;
use crate::simulation::world_state::WorldState;
use crate::display::renderable_objects::grid_hexagon::GridHexagon;
use crate::display::sdl::{RendererBuilderSDL, RendererSDL};
use crate::util::vec2::Vec2;


pub struct NullDisplay {}
pub struct NullRenderer {}
pub struct NullRendererBuilder {}

impl RendererBuilder for NullRendererBuilder {
    type Renderer = NullRenderer;

    fn set_display(&mut self, _name: &str, _width: u32, _height: u32) -> &mut Self {
        self
    }

    fn build(&mut self) -> NullRenderer {
        NullRenderer {}
    }
}

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
    type Renderer = NullRenderer;
    type RendererBuilder = NullRendererBuilder;

    fn render(&mut self, state: &WorldState, camera: &Camera, renderer: &mut Self::Renderer) {
        ()
    }

    fn create_renderer_builder(&mut self) -> Self::RendererBuilder {
        todo!()
    }

    fn create_renderer(&mut self, renderer_builder: &mut Self::RendererBuilder) -> Self::Renderer {
        todo!()
    }
}

pub struct SdlDisplay;

impl GameDisplay for SdlDisplay {
    type Renderer = RendererSDL;
    type RendererBuilder = RendererBuilderSDL;

    fn render(&mut self, state: &WorldState, camera: &Camera, renderer: &mut Self::Renderer) {

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
            hexagon.render(camera, renderer);
        }

        renderer.present();
    }

    fn create_renderer_builder(&mut self) -> Self::RendererBuilder {
        let mut sdl = match sdl2::init() {
            Ok(sdl) => sdl,
            Err(e) => panic!("Failed to initialize SDL: {}", e),
        };

        RendererBuilderSDL::new(&mut sdl)
    }

    fn create_renderer(&mut self, renderer_builder: &mut Self::RendererBuilder) -> Self::Renderer {
        renderer_builder.build()
    }
}
