use crate::collision::{
    check_bottom_collision, check_left_collision, check_right_collision, check_top_collision,
};
use crate::entity::{Player, Wall};
use tetra::graphics::Color;
use tetra::input::{is_key_down, Key};
use tetra::{graphics, Context, State};

pub struct GameState {
    player: Player,
    walls: [Wall; 6],
}

impl GameState {
    const MAP_WIDTH: f32 = 1000.0;
    const MAP_HEIGHT: f32 = 1000.0;

    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {
            player: Player::new(ctx, GameState::MAP_WIDTH / 2.0, GameState::MAP_HEIGHT / 2.0),
            walls: [
                Wall::new(ctx, 300.0, 300.0, 1, 16),
                Wall::new(ctx, 300.0, 300.0, 10, 1),
                Wall::new(ctx, 450.0, 300.0, 1, 16),
                Wall::new(ctx, 300.0, 520.0, 4, 1),
                Wall::new(ctx, 400.0, 520.0, 4, 1),
                Wall::new(ctx, 400.0, 440.0, 4, 1),
            ],
        })
    }

    pub fn handle_player_movement(&mut self, ctx: &mut Context) {
        if is_key_down(ctx, Key::W) && !self.player_top_going_to_collide() {
            self.player.move_up()
        }
        if is_key_down(ctx, Key::S) && !self.player_bottom_going_to_collide() {
            self.player.move_down()
        }
        if is_key_down(ctx, Key::A) && !self.player_left_going_to_collide() {
            self.player.move_left()
        }
        if is_key_down(ctx, Key::D) && !self.player_right_going_to_collide() {
            self.player.move_right()
        }
    }

    fn player_top_going_to_collide(&self) -> bool {
        for wall in &self.walls {
            if check_top_collision(wall, &self.player) {
                return true;
            }
        }
        return false;
    }
    fn player_bottom_going_to_collide(&self) -> bool {
        for wall in &self.walls {
            if check_bottom_collision(wall, &self.player) {
                return true;
            }
        }
        return false;
    }
    fn player_left_going_to_collide(&self) -> bool {
        for wall in &self.walls {
            if check_left_collision(wall, &self.player) {
                return true;
            }
        }
        return false;
    }
    fn player_right_going_to_collide(&self) -> bool {
        for wall in &self.walls {
            if check_right_collision(wall, &self.player) {
                return true;
            }
        }
        return false;
    }

    fn draw_walls(&mut self, ctx: &mut Context) {
        for i in 0..self.walls.len() {
            self.walls[i].draw(ctx);
        }
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.handle_player_movement(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        clear_window(ctx);
        self.draw_walls(ctx);
        self.player.draw(ctx);
        Ok(())
    }
}

pub fn clear_window(ctx: &mut Context) {
    graphics::clear(ctx, Color::rgb(1.0, 1.0, 1.0));
}
