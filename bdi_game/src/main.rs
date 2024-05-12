use bdi_game::{display::game_display::SdlDisplay, game::game::Game};

fn main() {
    let mut game = Game::init(SdlDisplay {});
    game.run();
}
