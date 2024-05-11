use sdl2::gfx::primitives::DrawRenderer;
use sdl2::rect::{Point, Rect};

use super::resource_manager::ResourceManager;

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

pub enum Filling {
    Texture(String),
    Color(sdl2::pixels::Color),
    None,
}

pub struct Hexagon {
    pub position: (i32, i32),
    pub vertices: ([i16; 6], [i16; 6]),
    pub size: u32,
    pub filling: Filling,
}

impl Hexagon {
    pub fn new(position: (i32, i32), size: u32) -> Self {
        let vertices = Self::create_vertices(position, size);
        Self {
            position,
            vertices,
            size,
            filling: Filling::None,
        }
    }

    fn create_vertices(position: (i32, i32), size: u32) -> ([i16; 6], [i16; 6]) {
        let a = size as f64 / 3.0_f64.sqrt();
        let x_0 = position.0 as f64;
        let y_0 = position.1 as f64;
        let hexagon_vertices_x: [i16; 6] = [
            x_0 as i16,
            (x_0 + a * 3.0_f64.sqrt() / 2.0) as i16,
            (x_0 + a * 3.0_f64.sqrt() / 2.0) as i16,
            x_0 as i16,
            (x_0 - a * 3.0_f64.sqrt() / 2.0) as i16,
            (x_0 - a * 3.0_f64.sqrt() / 2.0) as i16,

        ];
        let hexagon_vertices_y: [i16; 6] = [
            (y_0 - a) as i16,
            (y_0 - 0.5*a) as i16,
            (y_0 + 0.5*a) as i16,
            (y_0 + a) as i16,
            (y_0 + 0.5*a) as i16,
            (y_0 - 0.5*a) as i16,
        ];

        (hexagon_vertices_x, hexagon_vertices_y)
    }
    

}

impl Renderable for Hexagon {
    fn render(
        &self,
        renderer: &mut sdl2::render::Canvas<sdl2::video::Window>,
        res_manager: &ResourceManager,
    ) -> () {
        match &self.filling {
            Filling::None => (),
            Filling::Color(color) => renderer
                .filled_polygon(&self.vertices.0, &self.vertices.1, *color)
                .unwrap(),
            Filling::Texture(name) => {
                let a = self.size;
                let h = a as f64 / 3.0_f64.sqrt();
                let rect =
                    Rect::from_center(Point::new(self.position.0, self.position.1), a, h as u32);

                renderer
                    .copy_ex(
                        &res_manager.get_texture(name),
                        None,
                        rect,
                        0.0,
                        None,
                        false,
                        false,
                    )
                    .unwrap();
                renderer
                    .copy_ex(
                        &res_manager.get_texture(name),
                        None,
                        rect,
                        60.0,
                        None,
                        false,
                        false,
                    )
                    .unwrap();
                renderer
                    .copy_ex(
                        &res_manager.get_texture(name),
                        None,
                        rect,
                        120.0,
                        None,
                        false,
                        false,
                    )
                    .unwrap();
            }
        };
    }
}

pub trait Renderable {
    fn render(
        &self,
        renderer: &mut sdl2::render::Canvas<sdl2::video::Window>,
        res_manager: &ResourceManager,
    ) -> ();
}
