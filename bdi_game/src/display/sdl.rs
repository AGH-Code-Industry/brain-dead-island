pub use super::traits::*;
use sdl2::{
    self,
    gfx::primitives::DrawRenderer,
    image::LoadTexture,
    render::{SurfaceCanvas, WindowCanvas},
    video::WindowContext,
};

const HEXAGON_H: i16 = 20;
const HEXAGON_S: i16 = 50;
const HEXAGON_VERTICES_X: [i16; 6] = [
    HEXAGON_S - ((1.73 / 2.) * HEXAGON_H as f32) as i16,
    HEXAGON_S,
    HEXAGON_S + HEXAGON_H,
    HEXAGON_S + HEXAGON_H + ((1.73 / 2.) * HEXAGON_H as f32) as i16,
    HEXAGON_S + HEXAGON_H,
    HEXAGON_S,
];
const HEXAGON_VERTICES_Y: [i16; 6] = [
    HEXAGON_S + HEXAGON_H,
    HEXAGON_S,
    HEXAGON_S,
    HEXAGON_S + HEXAGON_H,
    HEXAGON_S + (2 * HEXAGON_H),
    HEXAGON_S + (2 * HEXAGON_H),
];

pub enum UnitSDLFillType {
    Texture(String),
    Color(sdl2::pixels::Color),
    None,
}

/// SDL unit
pub struct UnitSDL<'a> {
    pub position: &'a (i32, i32),
    pub size: u32,
    pub filling: UnitSDLFillType,
}

impl<'a> UnitSDL<'a> {
    pub fn new(position: &'a (i32, i32)) -> Self {
        Self {
            position,
            size: 100,
            filling: UnitSDLFillType::None,
        }
    }
}

/// Initializes video submodules, create window, handle canvas.
pub struct DisplayBuilderSDL {
    width: u32,
    heigth: u32,
    name: String,
    video: sdl2::VideoSubsystem,
}

/// Manages clusters and renders on screen.
pub struct DisplaySDL {
    builder: DisplayBuilderSDL,
    canvas: sdl2::render::WindowCanvas,
    texture_handler: sdl2::render::TextureCreator<WindowContext>,
}

impl DisplayBuilder for DisplayBuilderSDL {
    type Unit<'a> = UnitSDL<'a> where Self : 'a;
    type Display<'a> = DisplaySDL;

    fn build<'a>(self) -> Self::Display<'a> {
        let _image_context =
            sdl2::image::init(sdl2::image::InitFlag::PNG | sdl2::image::InitFlag::JPG).unwrap();

        let canvas = self
            .video
            .window(self.name.as_str(), self.width, self.heigth)
            .build()
            .expect("Cannot create window")
            .into_canvas()
            .build()
            .expect("Cannot create canvas");
        DisplaySDL {
            builder: self,
            texture_handler: canvas.texture_creator(),
            canvas,
        }
    }

    fn set_display(mut self, name: &str, width: u32, heigth: u32) -> Self {
        self.name = name.to_string();
        self.width = width;
        self.heigth = heigth;
        self
    }
}

impl DisplayBuilderSDL {
    pub fn new(context: &mut sdl2::Sdl) -> Self {
        // No point in catching this errors
        let video = context.video().expect("Cannot start video subsystem");

        Self {
            width: 0,
            heigth: 0,
            name: String::new(),
            video,
        }
    }
}

impl Display for DisplaySDL {
    type Unit<'a> = UnitSDL<'a> where Self : 'a;

    fn create_cluster<'a>(&self) -> super::structures::Cluster<Self::Unit<'a>> {
        super::structures::Cluster::new()
    }

    fn render<'a>(&mut self, cluster: &super::structures::Cluster<Self::Unit<'a>>) {
        self.canvas.set_draw_color(sdl2::pixels::Color::WHITE);
        for unit in &cluster.objects {
            let tile =
                sdl2::rect::Rect::new(unit.position.0, unit.position.1, unit.size, unit.size);
            match unit.filling {
                UnitSDLFillType::Color(x) => self.canvas.set_draw_color(x),
                UnitSDLFillType::Texture(_) => {}
                UnitSDLFillType::None => {}
            }
            // We should CATCH this error.
            // Add handling here.
            self.canvas.fill_rect(tile).unwrap();
        }

        self.canvas.present();
    }
}

impl DisplaySDL {
    pub fn direct_draw(&mut self, unit: &UnitSDL) {
        let tile = sdl2::rect::Rect::new(unit.position.0, unit.position.1, unit.size, unit.size);
        match &unit.filling {
            UnitSDLFillType::Color(x) => {
                self.canvas.set_draw_color(*x);
                self.canvas.fill_rect(tile).unwrap();
            }
            UnitSDLFillType::Texture(t) => {
                let texture = self.texture_handler.load_texture(t.as_str()).unwrap(); // TODO: get read of loading texture each frame!
                self.canvas.copy(&texture, None, tile).unwrap();
            }
            UnitSDLFillType::None => {}
        }
    }

    pub fn direct_draw_polygon(&mut self, unit: &UnitSDL) {
        let (x, y) = create_hexagon_vertices(unit);
        match unit.filling {
            UnitSDLFillType::Color(color) => {
                self.canvas.filled_polygon(&x, &y, color).unwrap();
            }
            UnitSDLFillType::Texture(_) => {}
            UnitSDLFillType::None => {}
        }
    }

    pub fn direct_flush(&mut self) {
        self.canvas.present();
    }
}

fn create_hexagon_vertices(unit: &UnitSDL) -> ([i16; 6], [i16; 6]) {
    let heigth = (unit.size / 2) as i16;
    let x = unit.position.0 as i16;
    let y = unit.position.1 as i16;
    let approx_triangle_h = ((1.72 / 2.) * heigth as f32) as i16;
    let hexagon_vertices_x: [i16; 6] = [
        x - approx_triangle_h,
        x,
        x + heigth,
        x + heigth + approx_triangle_h,
        x + heigth,
        x,
    ];
    let hexagon_vertices_y: [i16; 6] = [
        y + heigth,
        y,
        y,
        y + heigth,
        y + (2 * heigth),
        y + (2 * heigth),
    ];

    (hexagon_vertices_x, hexagon_vertices_y)
}
