use core::engine::Engine;

use benchmark::Benchmark;
use mygame::MyGame;

mod benchmark;
mod core;
mod model;
mod mygame;
mod render;

#[macro_use]
extern crate glium;
fn main() {
    #[cfg(feature = "benchmark")]
    {
        let mut engine = Engine::<Benchmark>::new();
        engine.run();
    }

    #[cfg(not(feature = "benchmark"))]
    {
        let mut engine = Engine::<MyGame>::new();
        engine.run();
    }
}
