use crate::entity::Player;
use tetra::graphics::Color;
use tetra::input::{is_key_down, Key};
use tetra::math::Vec2;
use tetra::{graphics, Context, State};

pub struct GameState {
    player: Player,
}

impl GameState {
    const MAP_WIDTH: f32 = 1000.0;
    const MAP_HEIGHT: f32 = 1000.0;

    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {
            player: Player::new(
                ctx,
                Vec2::new(GameState::MAP_WIDTH / 2.0, GameState::MAP_HEIGHT / 2.0),
            ),
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
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

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        clear_window(ctx);
        self.player.draw(ctx);
        Ok(())
    }
}

pub fn clear_window(ctx: &mut Context) {
    graphics::clear(ctx, Color::rgb(1.0, 1.0, 1.0));
}
