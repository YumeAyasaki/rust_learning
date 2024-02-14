mod game;
use game::game_state::Game;

use ggez::{ContextBuilder};
use ggez::event::run;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("game", "ggez")
        .build()
        .expect("Failed to create ggez context");
    let mut game = Game::new(&mut ctx);
    run(ctx, event_loop,  game)
}
