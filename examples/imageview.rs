use ggez::audio;
use ggez::audio::SoundSource;
use ggez::event;
use ggez::graphics::{self, Color};
use ggez::timer;
use ggez::{Context, GameResult};
use std::env;
use std::path;

struct MainState {
    a: i32,
    direction: i32,
    image: graphics::Image,
    rng: oorandom::Rand32,
}

impl MainState {
    fn draw_crazy_lines(&mut self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        let num_lines = 100;
        let mut colors = Vec::new();
        for _ in 0..num_lines {
            let r = self.rng.rand_u32() as u8;
            let b = self.rng.rand_u32() as u8;
            let g = self.rng.rand_u32() as u8;
            colors.push(Color::from((r, g, b, 255)));
        }

        let mut last_point = glam::Vec2::new(400.0, 300.0);
        let mut mb = graphics::MeshBuilder::new();
        for color in colors {
            let x = (self.rng.rand_i32() % 50) as f32;
            let y = (self.rng.rand_i32() % 50) as f32;
            let point = glam::Vec2::new(last_point.x + x, last_point.y + y);
            mb.line(&[last_point, point], 3.0, color)?;
            last_point = point;
        }
        let mesh = graphics::Mesh::from_raw(&ctx.gfx, mb.build());
        canvas.draw_mesh(mesh.clone(), None, (glam::Vec2::new(0.0, 0.0),));

        Ok(())
    }

    fn new(ctx: &mut Context) -> GameResult<MainState> {
        ctx.fs.print_all();

        let image = graphics::Image::from_path(&ctx.fs, &ctx.gfx, "/dragon1.png", true)?;

        ctx.gfx.add_font(
            "LiberationMono",
            graphics::FontData::from_path(&ctx.fs, "/LiberationMono-Regular.ttf")?,
        );
        let mut sound = audio::Source::new(&ctx.fs, &ctx.audio, "/sound.ogg")?;

        // "detached" sounds keep playing even after they are dropped
        let _ = sound.play_detached(&ctx.audio);

        let rng = oorandom::Rand32::new(271828);

        let s = MainState {
            a: 0,
            direction: 1,
            image,
            rng,
        };

        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;
        while ctx.time.check_update_time(DESIRED_FPS) {
            self.a += self.direction;
            if self.a > 250 || self.a <= 0 {
                self.direction *= -1;

                println!("Delta frame time: {:?} ", ctx.time.delta());
                println!("Average FPS: {}", ctx.time.fps());
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let c = self.a as u8;
        let mut canvas = graphics::Canvas::from_frame(&ctx.gfx, Color::from([0.1, 0.2, 0.3, 1.0]));

        let color = Color::from((c, c, c, 255));
        let dest_point = glam::Vec2::new(0.0, 0.0);
        canvas.draw(self.image.clone(), (dest_point, 0.0, color));
        canvas.draw_text(
            &[graphics::Text::new()
                .font("LiberationMono")
                .text("Hello world!")
                .size(48.0)
                .color(color)],
            dest_point,
            0.0,
            graphics::TextLayout::tl_single_line(),
            0,
        );

        let dest_point2 = glam::Vec2::new(0.0, 256.0);
        let rectangle = graphics::Mesh::new_rectangle(
            &ctx.gfx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 256.0, 500.0, 32.0),
            Color::from((0, 0, 0, 255)),
        )?;
        canvas.draw_mesh(rectangle.clone(), None, (glam::Vec2::new(0.0, 0.0),));
        canvas.draw_text(
            &[graphics::Text::new()
                .font("LiberationMono")
                .text("This text is 32 pixels high")
                .size(32.0)
                .color(Color::WHITE)],
            dest_point2,
            0.0,
            graphics::TextLayout::tl_single_line(),
            0,
        );

        self.draw_crazy_lines(ctx, &mut canvas)?;
        canvas.finish(&mut ctx.gfx)?;

        timer::yield_now();
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("imageview", "ggez").add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
