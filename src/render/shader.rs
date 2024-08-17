use std::fs;

use glium::{backend::Facade, program::Program};

pub fn program_from_path<P: AsRef<std::path::Path>, F: ?Sized + Facade>(
    facade: &F,
    vertex: P,
    fragment: P,
    geometry: Option<P>,
) -> Program {
    let vert_src = fs::read_to_string(vertex).expect("vertex shader");
    let frag_src = fs::read_to_string(fragment).expect("fragment shader");
    let geom_src: Option<String> =
        geometry.map(|path| fs::read_to_string(path).expect("goemetry shader"));
    let geom_src_ref: Option<&str> = geom_src.as_deref();

    return Program::from_source(facade, &vert_src, &frag_src, geom_src_ref).unwrap();
}
