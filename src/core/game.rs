use glium::winit::keyboard::Key;

use super::context::Context;

pub trait Game: Sized {
    fn new(_: &mut Context) -> Self;
    fn start(&mut self, _: &mut Context) {}
    fn update(&mut self, _: &mut Context);
    fn render(&self, _: &mut Context);
    fn end(&mut self, _: &mut Context) {}

    fn on_key(&mut self, _: &mut Context, _: &Key) {}
    fn on_key_pressed(&mut self, _: &mut Context, _: &Key) {}
    fn on_key_released(&mut self, _: &mut Context, _: &Key) {}

    fn on_resize(&mut self, _: &mut Context) {}
}
