use tetra::graphics::Texture;
use tetra::math::Vec2;
use tetra::{graphics, Context};

pub struct Player {
    texture: Texture,
    position: Vec2<f32>,
}

impl Player {
    const MOVEMENT_SPEED: f32 = 10.0;

    pub fn new(ctx: &mut Context, x: f32, y: f32) -> Player {
        Player {
            texture: Texture::new(ctx, "./resources/tennis-ball.png").unwrap(),
            position: Vec2::new(x, y),
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

pub struct Wall {
    texture: Texture,
    position: Vec2<f32>,
    height: i32,
    width: i32,
}

impl Wall {
    pub fn new(ctx: &mut Context, x: f32, y: f32, width: i32, height: i32) -> Wall {
        Wall {
            texture: Texture::new(ctx, "./resources/stone.png").unwrap(),
            position: Vec2::new(x, y),
            height,
            width,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        for i in 0..self.width {
            let width_offset = 16 * i;
            for j in 1..self.height {
                let height_offset = 16 * j;
                let position = Vec2::new(
                    self.position.x + width_offset as f32,
                    self.position.y + height_offset as f32,
                );
                graphics::draw(ctx, &self.texture, position);
            }
            let position = Vec2::new(self.position.x + width_offset as f32, self.position.y);
            graphics::draw(ctx, &self.texture, position);
        }
    }
}
