use std::time::{Duration, Instant};

use cgmath::{Deg, Point3, SquareMatrix};
use glium::{buffer::Buffer, winit::keyboard::Key, Surface};

use crate::{
    core::{context::Context, game::Game},
    model::{
        model::Model,
        octree::{self, Node, Octree},
    },
    render::{camera::Camera, screen::Screen},
};

pub struct MyGame {
    camera: Camera,
    screen: Screen,
    model: Model,
    model_buffer: Buffer<[octree::Node]>,

    i: u32,
    last_time: Instant,
    frames: u32,
}

impl Game for MyGame {
    fn new(ctx: &mut Context) -> Self {
        let screen = Screen::new(
            ctx.window().display(),
            "res/shader/shader.vert",
            "res/shader/shader.frag",
            None,
        );

        let camera = Camera::new(Deg(45.0), ctx.window().aspect_ratio() as f32);

        let model = Model::new("res/model/3x3x3.ply");
        let octree = Octree::new(&model).serialize();

        let model_buffer = Buffer::new(
            ctx.window().display(),
            octree.as_slice(),
            glium::buffer::BufferType::UniformBuffer,
            glium::buffer::BufferMode::Immutable,
        )
        .unwrap();

        MyGame {
            screen,
            camera,
            i: 0,
            model,
            model_buffer,
            last_time: Instant::now(),
            frames: 0,
        }
    }

    fn update(&mut self, ctx: &mut Context) {
        self.frames += 1;
        let elapsed = self.last_time.elapsed();
        if elapsed >= Duration::from_secs(1) {
            println!("FPS: {}", self.frames);
            self.frames = 0;
            self.last_time = Instant::now();
        }

        self.i += 1;

        let cam_x = (self.i as f32 / 200.0).sin() * 3.0;
        let cam_z = (self.i as f32 / 200.0).cos() * 3.0;
        let pos = cgmath::Point3::new(cam_x, 2.0, cam_z);

        self.camera.set_position(pos);
        self.camera.look_at(Point3::new(0.0, 0.0, 0.0));
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

        let uniforms = uniform! {
            screen_size: screen_size,
            view_inverse: view_inverse,
            proj_inverse: proj_inverse,
            Nodes: &self.model_buffer,
        };

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
