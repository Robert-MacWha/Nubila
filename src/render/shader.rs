use std::{fs, path::Path, time::Instant};

use glium::{
    backend::Facade,
    program::{ComputeShader, Program},
    uniforms::Uniforms,
};
use log::{error, info};

pub struct ComputeProgram {
    program: ComputeShader,
    work_groups: (u32, u32, u32),

    comp: String,
}

pub struct RenderProgram {
    program: Program,

    vert: String,
    frag: String,
}

impl ComputeProgram {
    pub fn new<F: ?Sized + Facade>(facade: &F, comp: &str, work_groups: (u32, u32, u32)) -> Self {
        let program = ComputeProgram::load(facade, comp).unwrap();
        Self {
            program,
            work_groups,
            comp: comp.to_string(),
        }
    }

    pub fn reload<F: ?Sized + Facade>(&mut self, facade: &F) {
        let program = ComputeProgram::load(facade, &self.comp);

        if let Ok(program) = program {
            self.program = program;
        } else {
            error!("Failed to reload compute shader: {:?}", program);
        }
    }

    pub fn execute<U: Uniforms>(&self, uniforms: U) {
        self.program.execute(
            uniforms,
            self.work_groups.0,
            self.work_groups.1,
            self.work_groups.2,
        );
    }

    pub fn program(&self) -> &ComputeShader {
        &self.program
    }

    fn load<F: ?Sized + Facade>(facade: &F, path: &str) -> Result<ComputeShader, String> {
        let compute_src = preprocess_shader(path);
        let prog = ComputeShader::from_source(facade, &compute_src);
        return prog.map_err(|e| e.to_string());
    }
}

impl RenderProgram {
    pub fn new<F: ?Sized + Facade>(facade: &F, vert: &str, frag: &str) -> Self {
        let program = RenderProgram::load(facade, vert, frag).unwrap();
        Self {
            program,
            vert: vert.to_string(),
            frag: frag.to_string(),
        }
    }

    pub fn reload<F: ?Sized + Facade>(&mut self, facade: &F) {
        let program = RenderProgram::load(facade, &self.vert, &self.frag);

        if let Ok(program) = program {
            self.program = program;
        } else {
            error!("Failed to reload render shader: {:?}", program);
        }
    }

    pub fn program(&self) -> &Program {
        &self.program
    }

    fn load<F: ?Sized + Facade>(
        facade: &F,
        vertex: &str,
        fragment: &str,
    ) -> Result<Program, String> {
        let vert_src = preprocess_shader(vertex);
        let frag_src = preprocess_shader(fragment);

        let start = Instant::now();
        let prog = Program::from_source(facade, &vert_src, &frag_src, None);
        info!("Loaded program in {:?}", start.elapsed());

        return prog.map_err(|e| e.to_string());
    }
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
