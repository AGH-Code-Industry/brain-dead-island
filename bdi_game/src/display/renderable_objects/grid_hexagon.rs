use crate::display::camera::Camera;
use crate::display::sdl::Renderer;
use crate::display::traits::RenderableObject;
use crate::util::vec2::Vec2;
use log::info;
use sdl2::pixels::Color;

pub struct GridHexagon {
    pub center: Vec2,
    pub elevation: i32,
    pub size: f32,
}

impl RenderableObject for GridHexagon {
    fn render(&self, camera: &Camera, renderer: &mut dyn Renderer) {
        let mut vertices = Vec::new();
        for i in 0..6 {
            let angle = std::f32::consts::PI * (2.0 * i as f32) / 6.0;

            vertices.push(Vec2::new(
                self.center.x + angle.cos() * self.size,
                self.center.y + angle.sin() * self.size,
            ));
        }

        // TODO: Change this remapping process into something other than consecutive ifs
        let mut elevation_color = Color::RGB(84, 160, 227);

        if self.elevation < 140 {
            elevation_color = Color::RGB(84, 160, 227);
        } else if self.elevation < 170 {
            elevation_color = Color::RGB(255, 231, 125);
        } else if self.elevation < 190 {
            elevation_color = Color::RGB(104, 219, 72);
        } else if self.elevation < 220 {
            elevation_color = Color::RGB(24, 105, 32);
        } else if self.elevation < 254 {
            elevation_color = Color::RGB(100, 100, 100);
        } else {
            elevation_color = Color::RGB(245, 245, 245);
        }

        let vertices_screen = camera.world_to_screen_multiple(vertices);
        renderer.render_polygon(&vertices_screen, elevation_color);
    }
}
