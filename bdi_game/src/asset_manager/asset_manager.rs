pub struct AssetManager {}

impl AssetManager {
    const ASSET_DIR: &'static str = "assets";

    const MAP_SUBDIR: &'static str = "maps";
    const MAP_EXTENSION: &'static str = "png";

    pub fn get_map_path(map_name: &str) -> String {
        format!(
            "{}/{}/{}.{}",
            Self::ASSET_DIR,
            Self::MAP_SUBDIR,
            map_name,
            Self::MAP_EXTENSION
        )
    }
}
