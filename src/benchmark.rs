use std::{fs::File, io::Write, time::Instant};

use cgmath::{Angle, Deg, Point3, SquareMatrix};
use defer::defer;
use glium::{buffer::Buffer, Surface};
use plotters::{
    chart::ChartBuilder,
    prelude::{self, BitMapBackend, IntoDrawingArea},
    series::LineSeries,
    style::colors,
};

use crate::{
    core::{context::Context, game::Game},
    model::{model::Model, octree},
    render::{camera::Camera, screen::Screen},
};

struct FPSSnapshot {
    angle: Deg<f32>,
    fps: f32,
}

pub struct Benchmark {
    camera: Camera,
    screen: Screen,
    model_buffer: Buffer<[octree::Node]>,

    start: Instant,
    deg: Deg<f32>,

    fps_log: Vec<FPSSnapshot>,
}

const BENCH_DIR: &str = "bench/";
const BENCH_TARGET: &str = "monu1";
const BENCH_INCREMENT: f32 = 0.1;
const BENCH_DURATION: f32 = 720.0;
const OCTREE_ORIGIN: [f32; 3] = [-1.0, -1.0, -1.0];
const OCTREE_SIZE: f32 = 2.0;

impl Game for Benchmark {
    fn new(ctx: &mut Context) -> Self {
        // custom logger for benchmark
        log::info!("Benchmark load start");
        let start_time = Instant::now();
        defer!(log::info!(
            "Benchmark load end: elapsed={:?}ms",
            start_time.elapsed()
        ));

        let screen = Screen::new(
            ctx.window().display(),
            "res/shader/shader.vert",
            "res/shader/shader.frag",
        );

        let camera = Camera::new(Deg(45.0), ctx.window().aspect_ratio() as f32);

        let model_start = Instant::now();
        let model = Model::new(&format!("res/model/{}.ply", BENCH_TARGET));
        log::info!("Model load: elapsed={:?}", model_start.elapsed());

        let octree_start = Instant::now();
        let mut octree = octree::Octree::new(&model);
        log::info!("Octree build: elapsed={:?}", octree_start.elapsed());

        let optimize_start = Instant::now();
        octree.optimize();
        log::info!("Octree optimize: elapsed={:?}", optimize_start.elapsed());

        let serialize_start = Instant::now();
        let serialized = octree.serialize();
        log::info!("Octree serialize: elapsed={:?}", serialize_start.elapsed());

        let buffer_start = Instant::now();
        let model_buffer = Buffer::new(
            ctx.window().display(),
            serialized.as_slice(),
            glium::buffer::BufferType::UniformBuffer,
            glium::buffer::BufferMode::Immutable,
        )
        .unwrap();
        log::info!(
            "Buffer build: elapsed={:?} size={} bytes len={}",
            buffer_start.elapsed(),
            model_buffer.get_size(),
            model_buffer.len()
        );

        Benchmark {
            screen,
            camera,
            model_buffer,
            start: Instant::now(),
            deg: Deg(0.0),
            fps_log: Vec::new(),
        }
    }

    fn start(&mut self, _: &mut Context) {
        log::info!("Benchmark start");
        self.start = Instant::now();
    }

    fn update(&mut self, ctx: &mut Context) {
        if self.deg >= Deg(BENCH_DURATION) {
            ctx.exit();
        }

        // Update the camera
        self.deg += Deg(BENCH_INCREMENT);
        let cam_x = self.deg.cos() * 2.0;
        let cam_z = self.deg.sin() * 2.0;
        let pos = Point3::new(cam_x, 0.0, cam_z);
        self.camera.set_position(pos);
        self.camera.look_at(Point3::new(0.0, 0.0, 0.0));

        // Log the framerate
        let fps = ctx.time().fps() as f32;
        self.fps_log.push(FPSSnapshot {
            angle: self.deg,
            fps,
        });
    }

    fn render(&self, ctx: &mut Context) {
        let mut target = ctx.window().draw();
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
            Octree: &self.model_buffer,
            octree_origin: OCTREE_ORIGIN,
            octree_size: OCTREE_SIZE,
        };

        self.screen.draw(&mut target, uniforms);
        target.finish().expect("Window draw failed");
    }

    fn end(&mut self, _: &mut Context) {
        log::info!("Benchmark end: elapsed={:?}", self.start.elapsed());

        // smooth the fps log
        let mut smoothed = Vec::new();
        for i in 0..self.fps_log.len() {
            let mut sum = 0.0;
            let mut count = 0;
            for j in -5..5 {
                let idx = i as i32 + j;
                if idx >= 0 && idx < self.fps_log.len() as i32 {
                    sum += self.fps_log[idx as usize].fps;
                    count += 1;
                }
            }
            smoothed.push(FPSSnapshot {
                angle: self.fps_log[i].angle,
                fps: sum / count as f32,
            });
        }

        // safe the fps log
        let mut file = File::create(format!("{}fps.csv", BENCH_DIR)).unwrap();
        writeln!(file, "angle,fps").unwrap();
        for snapshot in smoothed.iter() {
            writeln!(file, "{},{}", snapshot.angle.0, snapshot.fps).unwrap();
        }

        // graph the fps log
        let plot_path = format!("{}fps.png", BENCH_DIR);
        let root = BitMapBackend::new(&plot_path, (1900, 600)).into_drawing_area();
        root.fill(&prelude::WHITE).unwrap();
        let root = root.titled("Nubila Benchmark", ("Arial", 50)).unwrap();

        let max_fps = smoothed.iter().map(|s| s.fps).fold(0.0, f32::max) as f64;
        let avg_fps = smoothed.iter().map(|s| s.fps).sum::<f32>() / smoothed.len() as f32;
        let x_spec = 0.0..BENCH_DURATION as f64;
        let y_spec = 0.0..max_fps;

        let caption = format!(
            "target={} | avg={} | timestamp={}",
            BENCH_TARGET,
            avg_fps,
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        );
        let mut chart = ChartBuilder::on(&root)
            .caption(&caption, ("Arial", 30))
            .set_left_and_bottom_label_area_size(50)
            .build_cartesian_2d(x_spec, y_spec)
            .unwrap();

        chart
            .configure_mesh()
            .x_desc("Angle")
            .y_desc("FPS")
            .draw()
            .unwrap();

        let series = LineSeries::new(
            smoothed.iter().map(|s| (s.angle.0 as f64, s.fps as f64)),
            &colors::RED,
        );
        chart.draw_series(series).unwrap();
    }
}
