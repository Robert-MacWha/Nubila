use cgmath::{Deg, SquareMatrix};
use glium::{winit::keyboard::Key, Surface};

use crate::{
    core::{context::Context, game::Game},
    render::{camera::Camera, screen::Screen},
};

pub struct MyGame {
    camera: Camera,
    screen: Screen,

    i: u32,
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

        let camera = Camera::new(Deg(45.0), ctx.window().aspect_ratio() as f32);

        MyGame {
            screen,
            camera,
            i: 0,
        }
    }

    fn update(&mut self, ctx: &mut Context) {
        self.i += 1;

        let cam_y = (self.i as f32 / 200.0).sin() * 0.5;
        let pos = cgmath::Vector3::new(0.0, cam_y, 0.0);

        self.camera.set_position(pos)
    }

    fn render(&self, ctx: &mut Context) {
        let mut target = ctx.window().start_draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let screen_size = (
            ctx.window().size().width as i32,
            ctx.window().size().height as i32,
        );

        let view_inverse: [[f32; 4]; 4] = self.camera.view_matrix().invert().unwrap().into();
        let proj_inverse: [[f32; 4]; 4] = self.camera.proj_matrix().invert().unwrap().into();
        let uniforms = uniform! {screen_size: screen_size, view_inverse: view_inverse, proj_inverse: proj_inverse};

        self.screen.draw(&mut target, uniforms);
        ctx.window().end_draw(target);
    }

    fn on_key_pressed(&mut self, ctx: &mut Context, key: &Key) {
        match key {
            Key::Character(c) if c == "r" => {
                self.screen.reload(ctx.window().display());
            }
            _ => {}
        }
    }

    fn on_resize(&mut self, ctx: &mut Context) {
        self.camera
            .set_aspect_ratio(ctx.window().aspect_ratio() as f32)
    }
}
