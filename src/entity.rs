use tetra::graphics::{Rectangle, Texture};
use tetra::math::Vec2;
use tetra::{graphics, Context};

pub struct Player {
    texture: Texture,
    position: Vec2<f32>,
    collision_box: Rectangle,
}

impl Player {
    const MOVEMENT_SPEED: f32 = 10.0;

    pub fn new(ctx: &mut Context, x: f32, y: f32) -> Player {
        Player {
            texture: Texture::new(ctx, "./resources/tennis-ball.png").unwrap(),
            position: Vec2::new(x, y),
            collision_box: Rectangle {
                height: 16.0,
                width: 16.0,
                y,
                x,
            },
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
        collision_box.intersects(&self.collision_box)
    }
}

pub struct Wall {
    texture: Texture,
    position: Vec2<f32>,
    height: i32,
    width: i32,
    collision_box: Rectangle,
}

impl Wall {
    pub fn new(ctx: &mut Context, x: f32, y: f32, width: i32, height: i32) -> Wall {
        Wall {
            texture: Texture::new(ctx, "./resources/stone.png").unwrap(),
            position: Vec2::new(x, y),
            collision_box: Rectangle {
                height: (height * 16) as f32,
                width: (width * 16) as f32,
                y,
                x,
            },
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

impl Collidable for &Wall {
    fn get_collision_box(self) -> Rectangle {
        return self.collision_box;
    }
    fn check_collision(self, collision_box: Rectangle) -> bool {
        collision_box.intersects(&self.collision_box)
    }
}

pub trait Collidable {
    fn get_collision_box(self) -> Rectangle;
    fn check_collision(self, collision_box: Rectangle) -> bool;
}

pub fn going_to_collide_top<F, S>(first: F, second: S) -> bool
where
    F: Collidable,
    S: Collidable,
{
    let mut second_collision_box = second.get_collision_box();
    second_collision_box.y += 5.0;
    first.check_collision(second_collision_box)
}

pub fn going_to_collide_bottom<F, S>(first: F, second: S) -> bool
where
    F: Collidable,
    S: Collidable,
{
    let mut second_collision_box = second.get_collision_box();
    second_collision_box.y -= 5.0;
    first.check_collision(second_collision_box)
}

pub fn going_to_collide_left<F, S>(first: F, second: S) -> bool
where
    F: Collidable,
    S: Collidable,
{
    let mut second_collision_box = second.get_collision_box();
    second_collision_box.x += 5.0;
    first.check_collision(second_collision_box)
}

pub fn going_to_collide_right<F, S>(first: F, second: S) -> bool
where
    F: Collidable,
    S: Collidable,
{
    let mut second_collision_box = second.get_collision_box();
    second_collision_box.x -= 5.0;
    first.check_collision(second_collision_box)
}
