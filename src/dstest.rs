use crate::prelude::{device, surface_config};

pub(crate) static mut DEPTH_STENCIL_PASS: Option<DepthPass> = None;
pub(crate) fn init_depth_pass() {
    unsafe {
        DEPTH_STENCIL_PASS = Some(DepthPass::new())
    }
}

pub(crate) fn depth_pass() -> &'static DepthPass {
    unsafe {
        DEPTH_STENCIL_PASS.as_ref().unwrap()
    }
}

pub(crate) fn mut_depth_pass() -> &'static mut DepthPass {
    unsafe {
        DEPTH_STENCIL_PASS.as_mut().unwrap()
    }
}

#[derive(Debug)]
pub(crate) struct DepthPass {
    view: wgpu::TextureView,
}

impl DepthPass {
    fn new() -> Self {
        Self {
            view: Self::create_depth_stencil_texture()
        }
    }

    pub(crate) fn resize(&mut self) {
        self.view = Self::create_depth_stencil_texture();
    }

    fn create_depth_stencil_texture() -> wgpu::TextureView {
        let device = device();
        let config = surface_config();

        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Stencil Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor { label: Some("Depth Stencil Texture View"), ..Default::default()});

        view
    }

    pub(crate) fn view(&self) -> &wgpu::TextureView {
        &self.view
    }
}
