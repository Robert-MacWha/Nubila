use std::num::NonZeroU32;

use glium::{
    backend::glutin::Display,
    glutin::{
        config::ConfigTemplateBuilder,
        context::ContextAttributesBuilder,
        display::GetGlDisplay,
        prelude::{GlDisplay, NotCurrentGlContext},
        surface::{GlSurface, SurfaceAttributesBuilder, SwapInterval, WindowSurface},
    },
    winit::{self, dpi::PhysicalSize, raw_window_handle::HasWindowHandle},
    Frame,
};

pub struct Window {
    window: winit::window::Window,
    display: Display<WindowSurface>,

    size: PhysicalSize<u32>,
}

impl Window {
    pub fn new(event_loop: &winit::event_loop::EventLoop<()>, vsync: bool) -> Self {
        // Since SimpleWindowBuilder doesn't currently support setting vsync, we
        // need to construct the window and display manually.
        //
        // https://github.com/glium/glium/issues/2118

        let attributes = winit::window::Window::default_attributes()
            .with_title("Nubila")
            .with_inner_size(winit::dpi::PhysicalSize::new(1200, 720))
            .with_min_inner_size(winit::dpi::PhysicalSize::new(200, 120));

        // First we start by opening a new Window
        let display_builder =
            glutin_winit::DisplayBuilder::new().with_window_attributes(Some(attributes));
        let config_template_builder = ConfigTemplateBuilder::new();
        let (window, gl_config) = display_builder
            .build(event_loop, config_template_builder, |mut configs| {
                // Just use the first configuration since we don't have any special preferences here
                configs.next().unwrap()
            })
            .unwrap();
        let window = window.unwrap();

        // Now we get the window size to use as the initial size of the Surface
        let (width, height): (u32, u32) = window.inner_size().into();
        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            window
                .window_handle()
                .expect("couldn't obtain raw window handle")
                .into(),
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        // Finally we can create a Surface, use it to make a PossiblyCurrentContext and create the glium Display
        let surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        let context_attributes = ContextAttributesBuilder::new().build(Some(
            window
                .window_handle()
                .expect("couldn't obtain raw window handle")
                .into(),
        ));
        let current_context = Some(unsafe {
            gl_config
                .display()
                .create_context(&gl_config, &context_attributes)
                .expect("failed to create context")
        })
        .unwrap()
        .make_current(&surface)
        .unwrap();

        // Patch - allow vsync
        if !vsync {
            let _ = surface
                .set_swap_interval(&current_context, SwapInterval::DontWait)
                .expect("failed to disable vsync");
        }
        // \Patch

        let display = Display::from_context_surface(current_context, surface).unwrap();

        return Self {
            window,
            display,
            size: PhysicalSize::new(width, height),
        };
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }

        self.size = new_size;
        self.display.resize(new_size.into());
    }

    pub fn set_title(&mut self, name: &str) {
        self.window.set_title(name);
    }

    pub fn display(&mut self) -> &mut Display<WindowSurface> {
        return &mut self.display;
    }

    pub fn window(&mut self) -> &mut winit::window::Window {
        return &mut self.window;
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
