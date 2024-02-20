use ggez;
use ggez::mint::{Point2, Vector2};

use super::paddle::Paddle;

pub struct Game {
    computer: Paddle,
    player: Paddle,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Paddle::new(
                Point2 { x: 0.0, y: 0.0 },
                Vector2 { x: 0.0, y: 0.0 },
            ),
            computer: Paddle::new(
                Point2 { x: 0.0, y: 0.0 },
                Vector2 { x: 0.0, y: 0.0 },
            ),
        }
    }
}

impl ggez::event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        self.player.draw(_ctx);
        self.computer.draw(_ctx);
        Ok(())
    }
}