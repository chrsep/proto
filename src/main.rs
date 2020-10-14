use tetra::graphics::{self, Color, Texture};
use tetra::{Context, ContextBuilder, State};
use tetra::input::{self, Key};
use tetra::math::Vec2;

const Win_Width: f32 = 1024.0;
const Win_Height: f32 = 640.0;
const Pad_Speed: f32 = 10.0;

fn main() -> tetra::Result {
    ContextBuilder::new("Proto", Win_Width as i32, Win_Height as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
    }

struct Entity{
    texture: Texture,
    position: Vec2<f32>,
}

impl Entity{
    fn new(texture: Texture, position: Vec2<f32>) -> Entity{
        Entity{texture, position}
    }
}

struct GameState{
    player: Entity,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let p_texture = Texture::new(ctx, "./resources/tennis-ball.png")?;
        let p_position = 
            Vec2::new((Win_Width/2.0), (Win_Height /2.0));
              
            Ok(GameState{
                player: Entity::new(p_texture, p_position),
            })
    }
}


impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result{
        if input::is_key_down( ctx, Key::W) {
            self.player.position.y -= Pad_Speed;
        }

        if input::is_key_down( ctx, Key::S) {
            self.player.position.y += Pad_Speed;
        }

        if input::is_key_down( ctx, Key::A) {
            self.player.position.x -= Pad_Speed;
        }

        if input::is_key_down( ctx, Key::D) {
            self.player.position.x += Pad_Speed;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result{
        graphics::clear(ctx, Color::rgb(1.0, 1.0, 1.0));

        graphics::draw(ctx, &self.player.texture, self.player.position);
        
        Ok(())
    }
}
