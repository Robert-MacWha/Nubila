
use glium::{backend::Facade, Frame, Surface};

use super::{shader, vertex::Vertex};

pub struct Screen {
    program: glium::Program,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
}

impl Screen {
    pub fn new<P: AsRef<std::path::Path>, F: ?Sized + Facade>(
        facade: &F,
        vertex: P,
        fragment: P,
        geometry: Option<P>,
    ) -> Self {
        let program = shader::program_from_path(facade, vertex, fragment, geometry);

        let quad = vec![
            Vertex::new(-1.0, -1.0),
            Vertex::new(1.0, -1.0),
            Vertex::new(-1.0, 1.0),
            Vertex::new(1.0, 1.0),
        ];
        let vertex_buffer = glium::VertexBuffer::new(facade, &quad).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

        Screen {
            program,
            vertex_buffer,
            indices,
        }
    }

    pub fn draw<U: glium::uniforms::Uniforms>(&self, target: &mut Frame, uniforms: U) {
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
}
