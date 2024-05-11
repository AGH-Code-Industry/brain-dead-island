use sdl2::{image::LoadTexture, render::Texture, render::TextureCreator, video::WindowContext};
use std::collections::HashMap;
use std::fs;
use std::marker::PhantomData;

pub struct ResourceManager<'a> {
    textures_map: HashMap<String, Texture<'a>>,
    phantom: PhantomData<&'a ()>, // PhantomData to carry the lifetime
}

impl<'a> ResourceManager<'a> {
    pub fn new() -> ResourceManager<'a> {
        Self {
            textures_map: HashMap::new(),
            phantom: PhantomData,
        }
    }

    pub fn load_textures(&mut self, texture_creator: &'a TextureCreator<WindowContext>) {
        let textures_path = "bdi_game/resources/textures";

        if let Ok(entries) = fs::read_dir(textures_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(file_name) = path.file_stem() {
                        if let Some(file_name_str) = file_name.to_str() {
                            let texture = texture_creator.load_texture(&path).unwrap();
                            self.textures_map
                                .insert(String::from(file_name_str), texture);
                        }
                    }
                }
            }
        } else {
            println!("Could not read the texture directory.");
        }
    }

    pub fn get_texture(&self, name: &str) -> &Texture<'a> {
        self.textures_map.get(name).unwrap()
    }
}
