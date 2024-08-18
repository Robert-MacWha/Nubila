mod core;
mod model;
mod mygame;
mod render;

use core::engine::Engine;
use mygame::MyGame;

#[macro_use]
extern crate glium;
fn main() {
    let mut engine = Engine::<MyGame>::new();
    engine.run();
}
