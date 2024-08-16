use std::{borrow::Cow, fs};

pub fn load_shader_from_file<P: AsRef<std::path::Path>>(
    device: &wgpu::Device,
    path: P,
    stage: wgpu::naga::ShaderStage,
) -> wgpu::ShaderModule {
    let shader_code = fs::read_to_string(path).expect("Failed to read shader file");

    return device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("GLSL Shader"),
        source: wgpu::ShaderSource::Glsl {
            shader: Cow::Borrowed(&shader_code),
            stage,
            defines: Default::default(),
        },
    });
}
