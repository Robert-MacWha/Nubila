use super::time::Time;
use super::window::Window;

pub struct Context {
    window: Window,
    time: Time,
}

impl Context {
    pub fn new(window: Window) -> Self {
        let time = Time::new();
        return Context { window, time };
    }

    pub fn window(&mut self) -> &mut Window {
        return &mut self.window;
    }

    pub fn time(&mut self) -> &mut Time {
        return &mut self.time;
    }
}
