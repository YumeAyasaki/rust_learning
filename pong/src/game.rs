use ggez;

pub struct Game {

}

impl Game {
    pub fn new() -> Game {
        Game {}
    }

    pub fn run(&self) {
        println!("Game is running");
    }
}

impl ggez::event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }
}