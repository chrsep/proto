use crate::collision::Collidable;
use tetra::graphics::{Rectangle, Texture};
use tetra::math::{Aabr, Vec2};
use tetra::{graphics, Context};

// ===============================================================
// Player
// ===============================================================
pub struct Player {
    texture: Texture,
    position: Vec2<f32>,
    collision_box: Rectangle,
}

impl Player {
    const MOVEMENT_SPEED: f32 = 5.0;

    pub fn new(ctx: &mut Context, x: f32, y: f32) -> Player {
        let texture = Texture::new(ctx, "./resources/tennis-ball.png").unwrap();
        Player {
            position: Vec2::new(x, y),
            collision_box: Rectangle {
                height: texture.height() as f32,
                width: texture.width() as f32,
                y,
                x,
            },
            texture,
        }
    }

    pub fn move_up(&mut self) {
        self.position.y -= Player::MOVEMENT_SPEED;
        self.collision_box.y -= Player::MOVEMENT_SPEED;
    }

    pub fn move_down(&mut self) {
        self.position.y += Player::MOVEMENT_SPEED;
        self.collision_box.y += Player::MOVEMENT_SPEED;
    }

    pub fn move_left(&mut self) {
        self.position.x -= Player::MOVEMENT_SPEED;
        self.collision_box.x -= Player::MOVEMENT_SPEED;
    }

    pub fn move_right(&mut self) {
        self.position.x += Player::MOVEMENT_SPEED;
        self.collision_box.x += Player::MOVEMENT_SPEED;
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        graphics::draw(ctx, &self.texture, self.position);
    }
}

impl Collidable for &Player {
    fn get_collision_box(self) -> Rectangle {
        return self.collision_box;
    }
    fn check_collision(self, collision_box: Rectangle) -> bool {
        let self_aabb = Aabr {
            max: Vec2::new(
                self.collision_box.x + self.collision_box.width,
                self.collision_box.y + self.collision_box.height,
            ),
            min: Vec2::new(self.collision_box.x, self.collision_box.y),
        };
        let other_aabb = Aabr {
            max: Vec2::new(
                collision_box.x + collision_box.width,
                collision_box.y + collision_box.height,
            ),
            min: Vec2::new(collision_box.x, collision_box.y),
        };
        self_aabb.collides_with_aabr(other_aabb)
    }
}

// ===============================================================
// Wall
// ===============================================================
pub struct Wall {
    texture: Texture,
    position: Vec2<f32>,
    height: i32,
    width: i32,
    collision_box: Rectangle,
}

impl Wall {
    pub fn new(ctx: &mut Context, x: f32, y: f32, width: i32, height: i32) -> Wall {
        let texture = Texture::new(ctx, "./resources/stone.png").unwrap();
        Wall {
            position: Vec2::new(x, y),
            collision_box: Rectangle {
                height: (height * texture.height() + 4) as f32,
                width: (width * texture.width() + 4) as f32,
                y: y - 2.0,
                x: x - 2.0,
            },
            height,
            width,
            texture,
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

impl Collidable for &Wall {
    fn get_collision_box(self) -> Rectangle {
        return self.collision_box;
    }
    fn check_collision(self, collision_box: Rectangle) -> bool {
        let self_aabb = Aabr {
            max: Vec2::new(
                self.collision_box.x + self.collision_box.width,
                self.collision_box.y + self.collision_box.height,
            ),
            min: Vec2::new(self.collision_box.x, self.collision_box.y),
        };
        let other_aabb = Aabr {
            max: Vec2::new(
                collision_box.x + collision_box.width,
                collision_box.y + collision_box.height,
            ),
            min: Vec2::new(collision_box.x, collision_box.y),
        };
        self_aabb.collides_with_aabr(other_aabb)
    }
}
