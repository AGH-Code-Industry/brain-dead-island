use bdi_game::{display::game_display::SdlDisplay, game::game::Game};

fn main() {
    let mut game = Game::init(SdlDisplay::new("dupa", 1280, 720));
    game.run();
}
