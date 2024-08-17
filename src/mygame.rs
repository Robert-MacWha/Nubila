use glium::Surface;

use crate::{
    core::{context::Context, game::Game},
    render::screen::Screen,
};

pub struct MyGame {
    screen: Screen,
}

#[derive(Copy, Clone)]
struct FloatBuffer {
    data: [f32; 6],
}

implement_uniform_block!(FloatBuffer, data);

impl Game for MyGame {
    fn new(ctx: &mut Context) -> Self {
        let screen = Screen::new(
            ctx.window().display(),
            "res/shader/shader.vert",
            "res/shader/shader.frag",
            None,
        );

        MyGame { screen }
    }

    fn update(&mut self, ctx: &mut Context) {
        println!("Game updated");
    }

    fn render(&self, ctx: &mut Context) {
        let mut target = ctx.window().start_draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let screen_size = (
            ctx.window().size().width as i32,
            ctx.window().size().height as i32,
        );

        let float_data = FloatBuffer {
            data: [0.5, 0.1, 0.0, 0.0, 0.0, 0.0],
        };

        let buffer =
            glium::uniforms::UniformBuffer::new(ctx.window().display(), float_data).unwrap();

        self.screen.draw(
            &mut target,
            uniform! {screenSize: screen_size, floatBuffer: &buffer},
        );
        ctx.window().end_draw(target);
    }
}
