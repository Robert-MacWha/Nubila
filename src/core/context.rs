use super::time::Time;
use super::window::Window;

pub struct Context {
    window: Window,
    time: Time,

    exit_flag: bool,
}

impl Context {
    pub fn new(window: Window) -> Self {
        let time = Time::new();
        return Context {
            window,
            time,
            exit_flag: false,
        };
    }

    pub fn window(&mut self) -> &mut Window {
        return &mut self.window;
    }

    pub fn time(&mut self) -> &mut Time {
        return &mut self.time;
    }

    pub fn exit(&mut self) {
        self.exit_flag = true;
    }

    pub fn exiting(&self) -> bool {
        return self.exit_flag;
    }
}
