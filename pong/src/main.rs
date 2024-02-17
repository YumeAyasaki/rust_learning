use ggez;

mod game; // Add this line to import the `game` module

use game::Game;

fn main() {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build().unwrap();
    let state = Game::new();
    ggez::event::run(ctx, event_loop, state);
}
