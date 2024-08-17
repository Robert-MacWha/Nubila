use glium::winit::{event::KeyEvent, keyboard::Key};

use super::context::Context;

pub trait Game: Sized {
    fn new(ctx: &mut Context) -> Self;
    fn update(&mut self, ctx: &mut Context);
    fn render(&self, display: &mut Context);
    fn end(&mut self) {}

    fn on_key(&mut self, _: &mut Context, _: &Key) {}
    fn on_key_pressed(&mut self, _: &mut Context, _: &Key) {}
    fn on_key_released(&mut self, _: &mut Context, _: &Key) {}

    fn on_resize(&mut self, _: &mut Context) {}
}
