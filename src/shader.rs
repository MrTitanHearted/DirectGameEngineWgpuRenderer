use crate::common::*;

use std::collections::HashMap;

static mut LOADED_SHADERS: Option<HashMap<String, Shader>> = None;

pub fn shaders() -> &'static mut Vec<wgpu::ShaderModule> {
    unsafe { SHADERS.as_mut() }
}

pub fn shader(id: usize) -> &'static wgpu::ShaderModule {
    unsafe { &SHADERS[id] }
}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq)]
pub struct Shader {
    pub id: usize,
}

impl Shader {
    pub fn new(path: &str) -> std::io::Result<Self> {
        if let Some(shaders) = unsafe { &mut LOADED_SHADERS } {
            if let Some(shader) = shaders.get(path) {
                return Ok(*shader);
            } else {
                let shader = Self::load_from_file(path)?;
                shaders.insert(path.to_string(), shader);
                Ok(shader)
            }
        } else {
            let shader = Self::load_from_file(path)?;
            let mut shaders = HashMap::new();
            shaders.insert(path.to_string(), shader);
            unsafe {
                LOADED_SHADERS = Some(shaders);
            }
            Ok(shader)
        }
    }

    fn load_from_file(path: &str) -> std::io::Result<Self> {
        let id = shaders().len();
        use std::io::prelude::*;
        let device = device();
        let mut source = String::new();

        std::fs::File::open(path)
            .expect(&format!("System cannot find specified shader: {path}"))
            .read_to_string(&mut source)
            ?;

        shaders().push(device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(&source)),
        }));

        Ok(Shader { id })
    }

    pub(crate) fn module(&self) -> &'static wgpu::ShaderModule {
        shader(self.id)
    }
}
