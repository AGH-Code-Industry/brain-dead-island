use bdi_game::display::{
    hexagon::Hexagon,
    renderable::{Filling, Renderable},
    resource_manager::ResourceManager,
};
use sdl2::{event::Event, sys::SDL_Quit, pixels::Color};

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

    let mut hexagon = Hexagon::new((500, 100), 100);
    hexagon.filling = Filling::Color(Color::RGB(0, 102, 0));

    hexagon.render(&mut renderer, &res_manager);
    renderer.present();

    loop {
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
