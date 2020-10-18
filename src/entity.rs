use tetra::graphics::Texture;
use tetra::math::Vec2;
use tetra::{graphics, Context};

pub struct Player {
    texture: Texture,
    position: Vec2<f32>,
}

impl Player {
    const MOVEMENT_SPEED: f32 = 10.0;

    pub fn new(ctx: &mut Context, position: Vec2<f32>) -> Player {
        Player {
            texture: Texture::new(ctx, "./resources/tennis-ball.png").unwrap(),
            position,
        }
    }

    pub fn move_up(&mut self) {
        self.position.y -= Player::MOVEMENT_SPEED;
    }

    pub fn move_down(&mut self) {
        self.position.y += Player::MOVEMENT_SPEED;
    }

    pub fn move_left(&mut self) {
        self.position.x -= Player::MOVEMENT_SPEED;
    }

    pub fn move_right(&mut self) {
        self.position.x += Player::MOVEMENT_SPEED;
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        graphics::draw(ctx, &self.texture, self.position);
    }
}
