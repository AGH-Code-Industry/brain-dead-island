use image::GrayImage;
use std::cmp::{max, min};

pub struct Sampler2D {
    height_map: GrayImage,
    side_len: usize,
}

impl Sampler2D {
    pub fn new(height_map: GrayImage, side_len: usize) -> Sampler2D {
        Sampler2D {
            height_map,
            side_len,
        }
    }

    pub fn sample_hexagonal_axial(&self, q: i32, r: i32) -> i32 {
        let scale = self.height_map.width() as f32 / self.side_len as f32;

        let offset_q_x = scale * q as f32;
        let offset_q_y = 0f32;

        let offset_r_x = scale * 0.5 * r as f32;
        let offset_r_y = scale * 3.0f32.sqrt() / 2.0 * r as f32;

        let x = (offset_q_x + offset_r_x) * 1.5;
        let y = (offset_q_y + offset_r_y) / 2.0;

        // Linear interpolation
        let x0 = max(min(x.floor() as u32, self.height_map.width() - 1), 0);
        let x1 = max(min(x.ceil() as u32, self.height_map.width() - 1), 0);
        let y0 = max(min(y.floor() as u32, self.height_map.height() - 1), 0);
        let y1 = max(min(y.ceil() as u32, self.height_map.height() - 1), 0);

        let x0y0 = self.height_map.get_pixel(x0, y0)[0] as f32;
        let x1y0 = self.height_map.get_pixel(x1, y0)[0] as f32;
        let x0y1 = self.height_map.get_pixel(x0, y1)[0] as f32;
        let x1y1 = self.height_map.get_pixel(x1, y1)[0] as f32;

        let x0y = x0y0 + (x0y1 - x0y0) * (y - y0 as f32);
        let x1y = x1y0 + (x1y1 - x1y0) * (y - y0 as f32);

        (x0y + (x1y - x0y) * (x - x0 as f32)) as i32
    }
}
