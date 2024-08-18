use glium::{
    backend::glutin::Display,
    glutin::{config::ConfigTemplateBuilder, surface::WindowSurface},
    winit::{self, dpi::PhysicalSize},
    Frame,
};

pub struct Window {
    window: winit::window::Window,
    display: Display<WindowSurface>,

    size: PhysicalSize<u32>,
}

impl Window {
    pub fn new(event_loop: &winit::event_loop::EventLoop<()>) -> Self {
        let (window, display) =
            glium::backend::glutin::SimpleWindowBuilder::new().build(event_loop);

        window.set_title("Nubila");
        let size = window.inner_size();
        return Window {
            window,
            display,
            size,
        };
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }

        self.size = new_size;
        self.display.resize(new_size.into());
    }

    pub fn display(&mut self) -> &mut Display<WindowSurface> {
        return &mut self.display;
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        return self.size;
    }

    pub fn aspect_ratio(&self) -> f64 {
        return self.size.width as f64 / self.size.height as f64;
    }

    pub fn start_draw(&mut self) -> Frame {
        return self.display.draw();
    }

    pub fn end_draw(&self, f: Frame) {
        f.finish().unwrap();
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }
}
