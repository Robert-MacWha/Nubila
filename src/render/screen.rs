use glium::{backend::Facade, uniforms::Uniforms, Surface};

use super::{shader::RenderProgram, vertex::Vertex};

/// Screen is a struct that represents a screen in the game.
/// It contains a shader program and geometry to cover the full window.
pub struct Screen {
    program: RenderProgram,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
}

impl Screen {
    pub fn new<F: ?Sized + Facade>(facade: &F, vertex: &str, fragment: &str) -> Self {
        let program = RenderProgram::new(facade, vertex, fragment);

        let quad = vec![
            Vertex::new(-1.0, -1.0),
            Vertex::new(1.0, -1.0),
            Vertex::new(-1.0, 1.0),
            Vertex::new(1.0, 1.0),
        ];
        let vertex_buffer = glium::VertexBuffer::new(facade, &quad).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

        return Screen {
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
                self.program.program(),
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }

    pub fn reload<F: ?Sized + Facade>(&mut self, facade: &F) {
        self.program.reload(facade);
    }
}
