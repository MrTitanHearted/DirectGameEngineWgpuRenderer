use crate::common::*;

#[derive(Debug, Clone, Copy, Default, Hash)]
pub struct Texture2D {
    view: usize,
}

impl Texture2D {
    pub fn new(path: &str) -> Self {
        use wgpu::util::DeviceExt;

        let device = device();
        let queue = queue();

        let image = image::open(path).unwrap();
        let image = image.to_rgba8();
        let (width, height) = image.dimensions();

        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture_with_data(
            queue,
            &wgpu::TextureDescriptor {
                label: None,
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            },
            &image,
        );

        let view = texture_views().len();
        texture_views().push(texture.create_view(&wgpu::TextureViewDescriptor::default()));

        Self { view }
    }

    pub(crate) fn view(&self) -> &'static wgpu::TextureView {
        texture_view(self.view)
    }
}
