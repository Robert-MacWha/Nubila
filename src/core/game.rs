use super::context::Context;

pub trait Game: Sized {
    fn new(ctx: &mut Context) -> Self;
    fn update(&mut self, ctx: &mut Context);
    fn render(&self, display: &mut Context);
    fn end(&mut self) {}
}
