use crate::entity::{Player, Wall};
use tetra::graphics::Color;
use tetra::input::{is_key_down, Key};
use tetra::{graphics, Context, State};

pub struct GameState {
    player: Player,
    wall: Wall,
}

impl GameState {
    const MAP_WIDTH: f32 = 1000.0;
    const MAP_HEIGHT: f32 = 1000.0;

    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {
            player: Player::new(ctx, GameState::MAP_WIDTH / 2.0, GameState::MAP_HEIGHT / 2.0),
            wall: Wall::new(ctx, 300.0, 300.0, 2, 4),
        })
    }

    pub fn handle_player_movement(&mut self, ctx: &mut Context) {
        if is_key_down(ctx, Key::W) {
            self.player.move_up()
        }
        if is_key_down(ctx, Key::S) {
            self.player.move_down()
        }
        if is_key_down(ctx, Key::A) {
            self.player.move_left()
        }
        if is_key_down(ctx, Key::D) {
            self.player.move_right()
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
        self.wall.draw(ctx);
        self.player.draw(ctx);
        Ok(())
    }
}

pub fn clear_window(ctx: &mut Context) {
    graphics::clear(ctx, Color::rgb(1.0, 1.0, 1.0));
}
