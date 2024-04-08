use bdi_game::display::{self, traits::*, sdl::UnitSDLFillType};
use sdl2::{self, image::LoadTexture};

fn main() {
    let mut sdl = sdl2::init().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let mut rend = display::sdl::DisplayBuilderSDL::new(&mut sdl)
        .set_display("game", 500, 500)
        .build();

    let poss = [(34, 144), (20, 54), (200, 200)];
    let mut game_objs: Vec<display::sdl::UnitSDL> = poss
        .iter()
        .map(|x| display::sdl::UnitSDL::new(&x))
        .collect();
    game_objs.iter_mut().enumerate().for_each(|(i, x)| {
        x.filling = UnitSDLFillType::Texture(
            "grass.png".to_string()
        )
    });


    loop {
        for unit in &game_objs{
            rend.direct_draw(&unit);
        }
        rend.direct_flush();
        match event_pump.poll_event() {
            Some(sdl2::event::Event::Quit { .. }) => break,
            _ => continue,
        }
    }
}
