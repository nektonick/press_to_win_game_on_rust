use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{conf, Context, ContextBuilder, GameError, GameResult};

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    bg_color: Color,
    // Your state here...
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        let game = MyGame {
            bg_color: Color::RED,
            // ...
        };

        let fullscreen_type = conf::FullscreenType::Windowed;
        let _ = _ctx.gfx.set_fullscreen(fullscreen_type);
        _ctx.gfx.set_resizable(true);

        return game;
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, self.bg_color);
        // Draw code here...

        let red_text_part = graphics::TextFragment::new("You ").color(Color::GREEN);
        let blue_text_part = graphics::TextFragment::new("loose!").color(Color::BLUE);
        let mut text = graphics::Text::new("");
        text.add(red_text_part);
        text.add(blue_text_part);
        text.set_scale(64.);

        // Text is drawn from the top-left corner.
        //let offset = self.frames as f32 / 10.0;
        let h = ctx.gfx.window().outer_size().height;
        let w = ctx.gfx.window().inner_size().width;
        let dest_point = ggez::glam::Vec2::new((w / 2) as f32, (h / 2) as f32);

        canvas.draw(&text, dest_point);

        let new_color = Color::new(
            (self.bg_color.r + 0.01) % 1f32,
            self.bg_color.g,
            self.bg_color.b,
            self.bg_color.a,
        );
        self.bg_color = new_color;
        canvas.finish(ctx)
    }
}

struct State {}

impl EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
}
