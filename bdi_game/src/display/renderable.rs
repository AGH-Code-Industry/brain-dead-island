use crate::display::resource_manager::ResourceManager;

pub enum Filling {
    Texture(String),
    Color(sdl2::pixels::Color),
    None,
}

pub trait Renderable {
    fn render(
        &mut self,
        renderer: &mut sdl2::render::Canvas<sdl2::video::Window>,
        res_manager: &ResourceManager,
    ) -> ();
}
