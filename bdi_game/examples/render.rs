use bdi_game::render::{self, traits::*};

fn main(){

    let mut rend = render::sdl::RenderBuilderSDL::new()
        .window("game", 500, 500)
        .build();

    let mut cluster = rend.create_cluster();
    let game_obj = render::structures::Unit{};
    cluster.add(game_obj);
    loop {
        rend.render(&cluster);
    }

}
