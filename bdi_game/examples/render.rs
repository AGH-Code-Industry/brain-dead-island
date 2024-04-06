use bdi_game::display::{self, traits::*};
use sdl2;

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
        x.color = Some(sdl2::pixels::Color::RGB(
            20 + 50 * i as u8,
            100 + 30 * i as u8,
            55 * i as u8,
        ))
    });

    let mut cluster = rend.create_cluster();

    cluster.bulk_add(game_objs);
    loop {
        rend.render(&cluster);
        match event_pump.poll_event() {
            Some(sdl2::event::Event::Quit { .. }) => break,
            _ => continue,
        }
    }
}
