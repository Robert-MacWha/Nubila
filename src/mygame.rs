use std::{
    thread,
    time::{Duration, Instant},
    vec,
};

use cgmath::{Deg, Point3, SquareMatrix};
use glium::{
    buffer::{Buffer, BufferMode, BufferType},
    texture::{RawImage2d, UncompressedUintFormat, UnsignedTexture2d},
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter, UniformBuffer},
    winit::keyboard::Key,
};
use image::{ImageBuffer, Rgba};
use log::info;

use crate::{
    core::{context::Context, game::Game},
    model::{
        model::Model,
        octree::{self, Octree},
    },
    render::{camera::Camera, screen::Screen},
};

const MAX_NODES: usize = 100000;

pub struct MyGame {
    camera: Camera,

    intermediate_texture: UnsignedTexture2d,
    intermediate_screen: Screen,
    window_screen: Screen,
    model_buffer: Buffer<[octree::Node]>,
    node_render_buffer: Buffer<[u32]>,

    i: u32,
    last_time: Instant,
    frames: u32,
}

impl Game for MyGame {
    fn new(ctx: &mut Context) -> Self {
        let camera = Camera::new(Deg(45.0), ctx.window().aspect_ratio() as f32);

        // screens
        let start = Instant::now();
        let intermediate_screen = Screen::new(
            ctx.window().display(),
            "res/shader/screen.vert",
            "res/shader/pixel_to_voxel.frag",
        );

        let window_screen = Screen::new(
            ctx.window().display(),
            "res/shader/screen.vert",
            "res/shader/pixel_paint.frag",
        );
        info!("Loaded screens in {:?}", start.elapsed());

        // texture
        let size = ctx.window().size();
        let intermediate_texture = UnsignedTexture2d::empty_with_format(
            ctx.window().display(),
            UncompressedUintFormat::U8U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            size.width,
            size.height,
        )
        .unwrap();

        // buffers
        let start = Instant::now();
        let model = Model::new("res/model/monu2.ply");
        info!("Loaded model in {:?}", start.elapsed());

        let mut octree = Octree::new(&model);
        octree.optimize();
        let serialized = octree.serialize();

        let model_buffer = Buffer::new(
            ctx.window().display(),
            serialized.as_slice(),
            glium::buffer::BufferType::UniformBuffer,
            glium::buffer::BufferMode::Immutable,
        )
        .unwrap();
        info!(
            "model_buffer: len={} size={:#?} bytes",
            model_buffer.len(),
            model_buffer.get_size()
        );

        let node_render_buffer = Buffer::empty_array(
            ctx.window().display(),
            BufferType::ShaderStorageBuffer,
            MAX_NODES,
            BufferMode::Immutable,
        )
        .unwrap();

        MyGame {
            camera,
            intermediate_texture,
            intermediate_screen,
            window_screen,
            model_buffer,
            node_render_buffer,
            last_time: Instant::now(),
            frames: 0,
            i: 0,
        }
    }

    fn update(&mut self, ctx: &mut Context) {
        self.frames += 1;
        let elapsed = self.last_time.elapsed();
        if elapsed >= Duration::from_secs(1) {
            info!("FPS: {}", self.frames);
            self.frames = 0;
            self.last_time = Instant::now();
        }

        self.i += 1;

        let cam_x = (self.i as f32 / 200.0).sin() * 3.0;
        let cam_z = (self.i as f32 / 200.0).cos() * 3.0;
        let pos = cgmath::Point3::new(cam_x, 1.0, cam_z);

        self.camera.set_position(pos);
        self.camera.look_at(Point3::new(0.0, 0.0, 0.0));
        thread::sleep(Duration::from_millis(10));
    }

    fn render(&self, ctx: &mut Context) {
        //* First pass - render voxels to texture
        let mut intermediate_target = self.intermediate_texture.as_surface();
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
            NodeRender: &self.node_render_buffer,
            octree_origin: [-1 as f32, -1 as f32, -1 as f32],
            octree_size: 2 as f32,
        };

        self.intermediate_screen
            .draw(&mut intermediate_target, uniforms);

        //* Second pass - render texture to screen
        let mut window_target = ctx.window().draw();
        let screen_size = (
            ctx.window().size().width as u32,
            ctx.window().size().height as u32,
        );

        let sampler = self
            .intermediate_texture
            .sampled()
            .magnify_filter(MagnifySamplerFilter::Nearest)
            .minify_filter(MinifySamplerFilter::Nearest);

        let uniforms = uniform! {
            screen_size: screen_size,
            voxel_map: sampler,
            NodeRender: &self.node_render_buffer,
        };

        self.window_screen.draw(&mut window_target, uniforms);
        window_target.finish().expect("Window draw failed");
    }

    fn on_key_pressed(&mut self, ctx: &mut Context, key: &Key) {
        match key {
            Key::Character(c) if c == "r" => {
                self.intermediate_screen.reload(ctx.window().display());
                self.window_screen.reload(ctx.window().display());
            }
            Key::Character(c) if c == "s" => {
                let raw_image: RawImage2d<u8> = self.intermediate_texture.read();

                let image_buffer: Option<ImageBuffer<Rgba<u8>, Vec<u8>>> = ImageBuffer::from_raw(
                    raw_image.width,
                    raw_image.height,
                    raw_image.data.into_owned(),
                );

                match image_buffer {
                    Some(image_buffer) => {
                        let saved = image_buffer.save("screenshot.png");
                        match saved {
                            Ok(_) => {
                                info!("Saved screenshot");
                            }
                            Err(e) => {
                                info!("Failed to save screenshot: {:?}", e);
                            }
                        }
                    }
                    None => {
                        info!("Failed to save screenshot");
                    }
                }
            }
            _ => {}
        }
    }

    fn on_resize(&mut self, ctx: &mut Context) {
        self.camera
            .set_aspect_ratio(ctx.window().aspect_ratio() as f32);

        let size = ctx.window().size();
        self.intermediate_texture = UnsignedTexture2d::empty_with_format(
            ctx.window().display(),
            UncompressedUintFormat::U8U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            size.width,
            size.height,
        )
        .unwrap();
    }
}
