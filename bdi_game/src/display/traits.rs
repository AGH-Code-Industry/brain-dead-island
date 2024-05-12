use crate::display::camera::Camera;
use crate::display::sdl::RendererBuilderSDL;
use crate::simulation::world_state::WorldState;
use crate::util::vec2::Vec2;
use sdl2::pixels::Color;

/**
 * RendererBuilder trait used for creating a new Renderer object.
 */
pub trait RendererBuilder {
    type Renderer: Renderer;

    fn set_display(&mut self, name: &str, width: u32, height: u32) -> &mut Self;
    fn build(&mut self) -> Self::Renderer;
}

/**
 * Renderer trait used for rendering various objects on the screen.
 * The Display trait is used by the RenderableObject trait to render objects on the screen.
 */
pub trait Renderer {
    fn render_polygon(&mut self, vertices: &Vec<Vec2>, color: Color);
    fn render_line(&mut self, start: Vec2, end: Vec2, color: Color);
    fn render_tex(&mut self, pos: Vec2, tex: &str);
    fn present(&mut self);
}

/**
 * GameDisplay trait used for rendering the game state on the screen based on WorldState.
 */
pub trait GameDisplay {
    type Renderer: Renderer;
    type RendererBuilder: RendererBuilder;

    fn render(&mut self, state: &WorldState, camera: &Camera, renderer: &mut Self::Renderer);
    fn create_renderer_builder(&mut self) -> Self::RendererBuilder;
    fn create_renderer(&mut self, renderer_builder: &mut Self::RendererBuilder) -> Self::Renderer;
}

/**
 * RenderableObject trait shared by all objects that are gonna be rendered on screen, Grid Hexagons, Textured Objects etc.
 * Render function will be called on all Renderable object in GameDisplay::render() function.
 */
pub trait RenderableObject {
    fn render(&self, camera: &Camera, display: &mut dyn Renderer);
}
