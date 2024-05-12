pub use super::traits::{Renderer, RendererBuilder};
use crate::util::vec2::Vec2;
use sdl2::pixels::Color;
use sdl2::{self, gfx::primitives::DrawRenderer, video::WindowContext};

#[derive(Clone)]
pub struct RendererBuilderSDL {
    width: u32,
    height: u32,
    name: String,
    video: sdl2::VideoSubsystem,
}

impl RendererBuilderSDL {
    pub fn new(context: &mut sdl2::Sdl) -> Self {
        // No point in catching this errors
        let video = context.video().expect("Cannot start video subsystem");

        Self {
            width: 0,
            height: 0,
            name: String::new(),
            video,
        }
    }
}

impl RendererBuilder for RendererBuilderSDL {
    type Renderer = RendererSDL;

    fn set_display(&mut self, name: &str, width: u32, height: u32) -> &mut Self {
        self.name = name.to_string();
        self.width = width;
        self.height = height;
        self
    }

    fn build(&mut self) -> RendererSDL {
        let _image_context =
            sdl2::image::init(sdl2::image::InitFlag::PNG | sdl2::image::InitFlag::JPG).unwrap();

        let canvas = self
            .video
            .window(self.name.as_str(), self.width, self.height)
            .build()
            .expect("Cannot create window")
            .into_canvas()
            .build()
            .expect("Cannot create canvas");

        RendererSDL {
            builder: self.clone(),
            texture_handler: canvas.texture_creator(),
            canvas,
        }
    }
}

/**
 * Renderer class for SDL2
 */
pub struct RendererSDL {
    builder: RendererBuilderSDL,
    canvas: sdl2::render::WindowCanvas,
    texture_handler: sdl2::render::TextureCreator<WindowContext>,
}

impl Renderer for RendererSDL {
    fn render_polygon(&mut self, vertices: &Vec<Vec2>, color: Color) {
        let mut x = Vec::new();
        let mut y = Vec::new();
        for vertex in vertices {
            x.push(vertex.x as i16);
            y.push(vertex.y as i16);
        }
        self.canvas.filled_polygon(&x, &y, color).unwrap();
    }

    fn render_tex(&mut self, pos: Vec2, tex: &str) {
        todo!()
    }

    fn render_line(&mut self, start: Vec2, end: Vec2, color: Color) {
        self.canvas
            .draw_line(
                (start.x as i32, start.y as i32),
                (end.x as i32, end.y as i32),
            )
            .unwrap();
    }

    fn present(&mut self) {
        self.canvas.present();
    }
}
