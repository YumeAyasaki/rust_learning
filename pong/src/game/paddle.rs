use ggez::mint::{Point2, Vector2};
use ggez::graphics;

use super::utils::Direction;

pub struct Paddle {
    position: Point2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    size: Vector2<f32>,
    direction: Direction,
}

impl Paddle {
    pub fn new(position: Point2<f32>, size: Vector2<f32>) -> Paddle {
        Paddle {
            position,
            velocity: Vector2 { x: 0.0, y: 0.0 },
            acceleration: Vector2 { x: 0.0, y: 0.0 },
            size,
            direction: Direction::Up,
        }
    }

    pub fn set_acceleration(&mut self, acceleration: Vector2<f32>) {
        self.acceleration = acceleration;
    }

    pub fn set_velocity(&mut self, velocity: Vector2<f32>) {
        self.velocity = velocity;
    }

    pub fn update(&mut self) {
        self.velocity.x += self.acceleration.x;
        self.velocity.y += self.acceleration.y;
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }

    pub fn draw(&self, ctx: &mut ggez::Context) {
        // Draw the paddle
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                self.position.x,
                self.position.y,
                self.size.x,
                self.size.y,
            ),
            graphics::Color::WHITE,
        )
        .unwrap();
    }
}