use image::GrayImage;

pub struct Map {
    pub side_len: usize,
    pub height_map: GrayImage,
    pub height_modifiers: GrayImage,
}