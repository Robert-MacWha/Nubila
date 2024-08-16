use crate::engine::EngineResources;

pub trait Game {
    fn start(&mut self);
    fn update(&mut self);
    fn render(&mut self, render_pass: &mut wgpu::RenderPass);
}
