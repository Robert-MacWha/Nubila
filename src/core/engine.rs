use glium::winit::{
    application::ApplicationHandler,
    event::{ElementState, StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

use super::{context::Context, game::Game};

use super::window::Window;

pub struct Engine<G: Game> {
    /// Consumed by the `run_app`
    event_loop: Option<EventLoop<()>>,
    context: Context,
    game: G,
}

impl<G: Game> Engine<G> {
    pub fn new() -> Self {
        let event_loop = glium::winit::event_loop::EventLoop::builder()
            .build()
            .expect("event loop");

        let window = Window::new(&event_loop, false);
        let mut context = Context::new(window);
        let game = G::new(&mut context);

        return Engine {
            event_loop: Some(event_loop),
            context,
            game,
        };
    }

    pub fn run(&mut self) {
        // I don't like this pattern, but it's the best way I've found to let Engine
        // create event_loop and call run_app with a mutable reference to self.
        let event_loop = self.event_loop.take().expect("event_loop uninitialized");
        let _ = event_loop.run_app(self);

        self.game.end();
    }

    pub fn frame(&mut self) {
        self.context.time().update();
        self.game.update(&mut self.context);
        self.game.render(&mut self.context);

        self.context.window().request_redraw();
    }
}

impl<G: Game> ApplicationHandler for Engine<G> {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, _cause: StartCause) {
        // Set control flow to Poll for continuous rendering
        event_loop.set_control_flow(glium::winit::event_loop::ControlFlow::Poll);
    }

    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                self.context.window().resize(size);
                self.game.on_resize(&mut self.context);
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if event.repeat {
                    return;
                }

                self.game.on_key(&mut self.context, &event.logical_key);
                match event.state {
                    ElementState::Pressed => self
                        .game
                        .on_key_pressed(&mut self.context, &event.logical_key),
                    ElementState::Released => self
                        .game
                        .on_key_released(&mut self.context, &event.logical_key),
                }
            }
            // WindowEvent::MouseInput {
            //     device_id,
            //     state,
            //     button,
            // } => {
            //     todo!();
            // }
            WindowEvent::RedrawRequested => {
                self.frame();
            }
            _ => {}
        }
    }
}
