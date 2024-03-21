use bdi_game::render::{self, traits::*};

fn main(){

    let rend = render::sdl::RenderBuilderSDL::new()
        .window("game", 500, 500)
        .build();

    //let game_obj = render::sdl::UnitSDL{

    //};
    //cluster.add(game_obj);
    loop {
        //rend.render(&cluster);
    }

}
