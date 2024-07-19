use crate::asset_manager::asset_manager::AssetManager;
use image::io::Reader as ImageReader;
use image::GrayImage;
use log::{debug, info};

pub struct MapLoader {}

impl MapLoader {
    pub fn map_from_image(map_name: &str) -> GrayImage {
        let map_path = AssetManager::get_map_path(map_name);

        let img = match ImageReader::open(map_path) {
            Ok(img) => img,
            Err(e) => {
                eprintln!("Error while loading map from file: {}", e);
                return GrayImage::new(0, 0);
            }
        };

        let img = match img.decode() {
            Ok(img) => img,
            Err(e) => {
                eprintln!("Error while decoding map format: {}", e);
                return GrayImage::new(0, 0);
            }
        };

        info!("Map '{}' loaded successfully!", map_name);
        debug!("Map format: {:?}", img.color());
        debug!("Map dimensions: {}x{}", img.width(), img.height());

        return img.into_luma8();
    }
}
