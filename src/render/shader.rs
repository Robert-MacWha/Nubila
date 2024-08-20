use std::{fs, path::Path, time::Instant};

use glium::{backend::Facade, program::Program, ProgramCreationError};
use log::info;

pub fn program_from_path<F: ?Sized + Facade>(
    facade: &F,
    vertex: &String,
    fragment: &String,
) -> Result<Program, String> {
    let vert_src = preprocess_shader(vertex);
    let frag_src = preprocess_shader(fragment);

    let start = Instant::now();
    let prog = Program::from_source(facade, &vert_src, &frag_src, None);
    info!("Loaded program in {:?}", start.elapsed());

    return prog.map_err(|e| e.to_string());
}

fn preprocess_shader<P: AsRef<Path>>(shader_path: P) -> String {
    let shader_path = shader_path.as_ref();
    let mut shader_code = fs::read_to_string(shader_path).expect("Shader file not found");

    // Look for #include directives and replace them with the contents of the included file
    while let Some(include_pos) = shader_code.find("#include") {
        let start = shader_code[include_pos..].find('"').unwrap() + include_pos + 1;
        let end = shader_code[start..].find('"').unwrap() + start;
        let include_path = shader_code[start..end].to_string();

        // Build the full path for the included file
        let include_file_path = shader_path.parent().unwrap().join(include_path);

        // Read the contents of the included file
        let include_file_contents =
            fs::read_to_string(include_file_path).expect("Included file not found");

        // Replace the #include directive with the file's contents
        shader_code.replace_range(include_pos..end + 1, &include_file_contents);
    }

    shader_code
}
