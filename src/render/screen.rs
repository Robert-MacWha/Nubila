use std::path::{Path, PathBuf};

use glium::{backend::Facade, uniforms::Uniforms, Frame, Surface};

use super::{shader, vertex::Vertex};

/// Screen is a struct that represents a screen in the game.
/// It contains a shader program and geometry to cover the full window.
pub struct Screen {
    program: glium::Program,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,

    vertex: String,
    fragment: String,
}

impl Screen {
    pub fn new<F: ?Sized + Facade>(facade: &F, vertex: &str, fragment: &str) -> Self {
        let vertex = vertex.to_string();
        let fragment = fragment.to_string();

        let program = shader::program_from_path(facade, &vertex, &fragment)
            .map_err(|e| {
                eprintln!("Error creating program: {}", e);
            })
            .unwrap();

        let quad = vec![
            Vertex::new(-1.0, -1.0),
            Vertex::new(1.0, -1.0),
            Vertex::new(-1.0, 1.0),
            Vertex::new(1.0, 1.0),
        ];
        let vertex_buffer = glium::VertexBuffer::new(facade, &quad).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

        return Screen {
            vertex,
            fragment,
            program,
            vertex_buffer,
            indices,
        };
    }

    pub fn draw<U: Uniforms, S: Surface>(&self, target: &mut S, uniforms: U) {
        target
            .draw(
                &self.vertex_buffer,
                self.indices,
                &self.program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }

    pub fn reload<F: ?Sized + Facade>(&mut self, facade: &F) {
        let program = shader::program_from_path(facade, &self.vertex, &self.fragment);

        match program {
            Ok(program) => {
                println!("Program reloaded");
                self.program = program;
            }
            Err(e) => {
                eprintln!("Error reloading program: {}", e.to_string());
            }
        }
    }
}
