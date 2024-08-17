use std::{error::Error, path::PathBuf};

use glium::{backend::Facade, Frame, Program, ProgramCreationError, Surface};

use super::{shader, vertex::Vertex};

pub struct Screen {
    program: glium::Program,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,

    vertex: PathBuf,
    fragment: PathBuf,
    geometry: Option<PathBuf>,
}

impl Screen {
    pub fn new<P: AsRef<std::path::Path>, F: ?Sized + Facade>(
        facade: &F,
        vertex: P,
        fragment: P,
        geometry: Option<P>,
    ) -> Self {
        let vertex = vertex.as_ref().to_path_buf();
        let fragment = fragment.as_ref().to_path_buf();
        let geometry = geometry.as_ref().map(|p| p.as_ref().to_path_buf());

        let program = shader::program_from_path(facade, &vertex, &fragment, geometry.as_ref())
            .expect("error loading program");

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
            geometry,
            program,
            vertex_buffer,
            indices,
        };
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

    pub fn reload<F: ?Sized + Facade>(&mut self, facade: &F) {
        let program =
            shader::program_from_path(facade, &self.vertex, &self.fragment, self.geometry.as_ref());

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
