use bdi_game::display::{
    rendering::{Filling, Hexagon, Renderable},
    resource_manager::ResourceManager,
};
use sdl2::event::Event;
use sdl2::sys::SDL_Quit;

struct GridSetup {
    shape: (u32, u32),
    cell_size: u32,
    cell_pos: Vec<(i32, i32)>,
}

impl GridSetup {
    pub fn new(shape: (u32, u32), cell_size: u32) -> Self {
        let cell_pos = Self::calc_pos(shape, cell_size);
        Self {
            shape,
            cell_size,
            cell_pos,
        }
    }

    pub fn get_pos(&self) -> &Vec<(i32, i32)> {
        &self.cell_pos
    }

    fn calc_pos(shape: (u32, u32), cell_size: u32) -> Vec<(i32, i32)> {
        let width = shape.0;
        let height = shape.1;
        let a = cell_size as f64 / 3.0_f64.sqrt();
        let x_offset = 3.0_f64.sqrt() * a;
        let y_offset = 1.5 * a;

        let mut cell_pos: Vec<(i32, i32)> = Vec::new();
        let starting_pos = (100.0, 100.0);
        let mut current_pos: (f64, f64) = starting_pos;

        for i in 0..height {
            for j in 0..width {
                cell_pos.push((current_pos.0.round() as i32, current_pos.1.round() as i32));
                current_pos.0 += x_offset;
            }
            current_pos.1 += y_offset;
            match i % 2 {
                0 => current_pos.0 = starting_pos.0 + x_offset / 2.0,
                1 => current_pos.0 = starting_pos.0,
                _ => (),
            };
        }

        cell_pos
    }
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let window = sdl
        .video()
        .unwrap()
        .window("Grid Render Example", 1920, 1080)
        .position_centered()
        .build()
        .unwrap();

    let mut renderer = window
        .into_canvas()
        .build()
        .expect("Failed to initialize renderer Canvas");

    let texture_creator = renderer.texture_creator();

    let mut res_manager = ResourceManager::new();
    res_manager.load_textures(&texture_creator);

    let setup = GridSetup::new((15, 10), 100);
    let mut cells: Vec<Hexagon> = setup
        .get_pos()
        .iter()
        .map(|x| Hexagon::new(*x, setup.cell_size))
        .collect();

    cells.iter_mut().for_each(|x| {
        x.filling = Filling::Texture(String::from("grass"));
    });

    loop {
        for cell in &cells {
            cell.render(&mut renderer, &res_manager);
        }
        renderer.present();
        renderer.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => unsafe {
                    SDL_Quit();
                    std::process::exit(0);
                },
                _ => (),
            }
        }
    }
}
