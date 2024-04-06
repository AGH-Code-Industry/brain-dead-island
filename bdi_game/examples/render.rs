use bdi_game::render::{self, traits::*};
use sdl2;

fn main() {
    let mut sdl = sdl2::init().unwrap();
    let mut rend = render::sdl::RenderBuilderSDL::new(&mut sdl)
        .window("game", 500, 500)
        .build();

    let pos: (i32, i32) = (34, 144);
    let mut game_obj = render::sdl::UnitSDL::new(&pos);
    game_obj.color = Some(sdl2::pixels::Color::RED);

    let mut cluster = rend.create_cluster();

    cluster.add(game_obj);
    loop {
        rend.render(&cluster);
    }
}
