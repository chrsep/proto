use crate::game_state::GameState;
use tetra::ContextBuilder;

mod collision;
mod entity;
mod game_state;

const WIN_WIDTH: f32 = 640.0;
const WIN_HEIGHT: f32 = 640.0;

fn main() -> tetra::Result {
    ContextBuilder::new("Proto", WIN_WIDTH as i32, WIN_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
