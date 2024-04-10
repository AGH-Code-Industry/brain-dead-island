use image::GrayImage;
use serde::{Deserialize, Serialize};
use crate::terrain_manager::world_object::WorldObject;

#[derive(Serialize, Deserialize)]
pub struct Map {
    pub side_len: usize,
    pub height_map: GrayImage,
    pub height_modifiers: GrayImage,
    pub objects: Vec<WorldObject>,
}