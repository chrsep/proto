use crate::collision::{
    going_to_collide_bottom, going_to_collide_left, going_to_collide_right, going_to_collide_top,
};
use crate::entity::{Player, Wall};
use tetra::graphics::Color;
use tetra::input::{is_key_down, Key};
use tetra::{graphics, Context, State};

pub struct GameState {
    player: Player,
    walls: [Wall; 1],
}

impl GameState {
    const MAP_WIDTH: f32 = 1000.0;
    const MAP_HEIGHT: f32 = 1000.0;

    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {
            player: Player::new(ctx, GameState::MAP_WIDTH / 2.0, GameState::MAP_HEIGHT / 2.0),
            walls: [Wall::new(ctx, 300.0, 300.0, 4, 4)],
        })
    }

    pub fn handle_player_movement(&mut self, ctx: &mut Context) {
        if is_key_down(ctx, Key::W) && !self.player_going_to_collide_with_walls_top() {
            self.player.move_up()
        }
        if is_key_down(ctx, Key::S) && !self.player_going_to_collide_with_walls_bottom() {
            self.player.move_down()
        }
        if is_key_down(ctx, Key::A) && !self.player_going_to_collide_with_walls_left() {
            self.player.move_left()
        }
        if is_key_down(ctx, Key::D) && !self.player_going_to_collide_with_walls_right() {
            self.player.move_right()
        }
    }

    pub fn player_going_to_collide_with_walls_top(&self) -> bool {
        for wall in &self.walls {
            if going_to_collide_top(wall, &self.player) {
                return true;
            }
        }
        return false;
    }
    pub fn player_going_to_collide_with_walls_bottom(&self) -> bool {
        for wall in &self.walls {
            if going_to_collide_bottom(wall, &self.player) {
                return true;
            }
        }
        return false;
    }
    pub fn player_going_to_collide_with_walls_left(&self) -> bool {
        for wall in &self.walls {
            if going_to_collide_left(wall, &self.player) {
                return true;
            }
        }
        return false;
    }
    pub fn player_going_to_collide_with_walls_right(&self) -> bool {
        for wall in &self.walls {
            if going_to_collide_right(wall, &self.player) {
                return true;
            }
        }
        return false;
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.handle_player_movement(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        clear_window(ctx);
        for i in 0..self.walls.len() {
            self.walls[i].draw(ctx);
        }
        self.player.draw(ctx);
        Ok(())
    }
}

pub fn clear_window(ctx: &mut Context) {
    graphics::clear(ctx, Color::rgb(1.0, 1.0, 1.0));
}
