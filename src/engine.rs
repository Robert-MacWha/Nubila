use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    window::WindowId,
};

use crate::{game::Game, input::Input, state::State};

pub trait EngineResources {
    fn input(&self) -> &Input;
}

pub struct Engine<'a> {
    event_loop: Option<EventLoop<()>>,
    state: State<'a>,
    input: Input,
}

impl<'a> Engine<'a> {
    pub async fn new() -> Self {
        env_logger::init();

        let event_loop = EventLoop::new().unwrap();
        let input = Input::new();

        // Create the state first, passing a reference to the window
        let state = State::new(&event_loop).await;

        Self {
            event_loop: Some(event_loop),
            state,
            input,
        }
    }

    pub fn run(&mut self, game: &mut dyn Game) {
        game.start();

        match self.event_loop.take() {
            Some(event_loop) => {
                let _ = event_loop.run(move |event, control_flow| {
                    self.event_handler(game, &event, control_flow);
                });
            }
            None => {
                panic!("EventLoop already taken!");
            }
        }
    }

    fn event_handler(
        &mut self,
        game: &mut dyn Game,
        event: &Event<()>,
        control_flow: &EventLoopWindowTarget<()>,
    ) {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } => {
                self.window_event_handler(game, event, window_id, control_flow);
            }
            Event::AboutToWait => {
                // RedrawRequested will only trigger once unless we manually
                // request it.
                self.state.window().request_redraw();
            }
            _ => {}
        }
    }

    fn window_event_handler(
        &mut self,
        game: &mut dyn Game,
        event: &WindowEvent,
        window_id: &WindowId,
        control_flow: &EventLoopWindowTarget<()>,
    ) {
        if *window_id != self.state.window().id() {
            return;
        }

        if self.input.update(event) {
            return;
        }

        match event {
            WindowEvent::CloseRequested => control_flow.exit(),
            WindowEvent::Resized(physical_size) => {
                self.state.resize(*physical_size);
            }
            WindowEvent::RedrawRequested => {
                game.update();
                match self.state.render(game) {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => self.state.resize(self.state.size),
                    // The system is out of memory, we should probably qui
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        control_flow.exit();
                    }
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            _ => {}
        }
    }
}

impl<'a> EngineResources for Engine<'a> {
    fn input(&self) -> &Input {
        &self.input
    }
}
