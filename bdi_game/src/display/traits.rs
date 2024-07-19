use crate::display::camera::Camera;
use crate::simulation::world_state::WorldState;
use crate::util::vec2::Vec2;
use sdl2::pixels::Color;

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
    fn render(&mut self, state: &WorldState, camera: &Camera);
}

/**
 * RenderableObject trait shared by all objects that are gonna be rendered on screen, Grid Hexagons, Textured Objects etc.
 * Render function will be called on all Renderable object in GameDisplay::render() function.
 */
pub trait RenderableObject {
    fn render<R: Renderer>(&self, camera: &Camera, display: &mut R);
}
