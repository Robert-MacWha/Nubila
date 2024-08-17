use super::window::Window;

pub struct Context {
    window: Window,
}

impl Context {
    pub fn new(window: Window) -> Self {
        return Context { window };
    }

    pub fn window(&mut self) -> &mut Window {
        return &mut self.window;
    }
}
