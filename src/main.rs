use nubila::{
    engine::{Engine, EngineResources},
    game::Game,
};

fn main() {
    // let mut game = MyGame {};

    // let mut engine = pollster::block_on(Engine::new(&mut game));
    // engine.run();
}

struct MyGame<'a> {
    engine: &'a mut dyn EngineResources,
}

impl Game for MyGame {
    fn start(&mut self) {
        println!("Game started!");
    }

    fn update(&mut self) {
        println!("Game updated!");
    }

    fn render(&mut self, render_pass: &mut wgpu::RenderPass) {
        println!("Game rendered!");
        // render_pass.set_pipeline(&self.render_pipeline);
        // render_pass.draw(0..3, 0..1);
    }
}
