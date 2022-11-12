use crate::common::*;

pub(crate) fn shaders() -> &'static mut Vec<wgpu::ShaderModule> {
    unsafe { SHADERS.as_mut() }
}

pub(crate) fn shader(id: usize) -> &'static wgpu::ShaderModule {
    unsafe { &SHADERS[id] }
}

#[derive(Debug, Clone, Copy, Default, Hash)]
pub struct Shader {
    pub id: usize,
}

impl Shader {
    pub fn new(path: &str) -> Self {
        let id = shaders().len();
        use std::io::prelude::*;
        let device = device();
        let mut source = String::new();

        std::fs::File::open(path)
            .unwrap()
            .read_to_string(&mut source)
            .unwrap();

        shaders().push(device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(&source)),
        }));

        Shader { id }
    }

    pub(crate) fn module(&self) -> &'static wgpu::ShaderModule {
        shader(self.id)
    }
}
