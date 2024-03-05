use bdi_game::{display::game_display::NullDisplay, game::game::Game};

fn main() {
    let mut game = Game::init(NullDisplay {});
    game.run();
}
