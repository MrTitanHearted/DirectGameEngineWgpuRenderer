use crate::common::*;
use wgpu::util::DeviceExt;

pub type RenderPipeline = usize;
pub type VertexBuffer = usize;
pub type IndexBuffer = usize;
pub type VertexBufferLayout = usize;
pub type UniformBindGroup = usize;
pub type UniformBindGroupEntry = usize;
pub type UniformBindGroupLayoutEntry = usize;
pub type TextureBindGroup = usize;
pub type TextureBindGroupEntry = usize;
pub type TextureBindGroupLayoutEntry = usize;

#[derive(Debug, Clone, Copy, Default, Hash)]
pub struct RenderBuffer {
    pipeline: RenderPipeline,

    shader: crate::shader::Shader,

    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,

    num_vertices: u32,
    num_indices: u32,

    vertex_buffer_layout: VertexBufferLayout,

    uniform_bind_group: UniformBindGroup,
    uniform_bind_group_entry: UniformBindGroupEntry,
    uniform_bind_group_layout_entry: UniformBindGroupLayoutEntry,

    texture_bind_group: TextureBindGroup,
    texture_bind_group_entry: TextureBindGroupEntry,
    texture_bind_group_layout_entry: TextureBindGroupLayoutEntry,
}

impl RenderBuffer {
    pub fn new() -> Self {
        let mut render_buffer = Self::default();

        render_buffer.uniform_bind_group_entry = uniform_bind_group_entries().len();
        render_buffer.uniform_bind_group_layout_entry = uniform_bind_group_layout_entries().len();

        render_buffer.texture_bind_group_entry = texture_2d_bind_group_entries().len();
        render_buffer.texture_bind_group_layout_entry =
            texture_2d_bind_group_layout_entries().len();

        uniform_bind_group_entries().push(Vec::new());
        uniform_bind_group_layout_entries().push(Vec::new());

        texture_2d_bind_group_entries().push(Vec::new());
        texture_2d_bind_group_layout_entries().push(Vec::new());

        render_buffer
    }

    pub fn with_shader(&mut self, shader: crate::shader::Shader) -> Self {
        self.shader = shader;

        self.to_owned()
    }

    pub fn with_vertices<
        T: bytemuck::Pod + bytemuck::Zeroable + crate::common::VertexBufferLayout,
    >(
        &mut self,
        vertices: &[T],
    ) -> Self {
        let buffer = device().create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        self.vertex_buffer = vertex_buffers().len();
        self.vertex_buffer_layout = vertex_buffer_layouts().len();
        self.num_vertices = vertices.len() as u32;

        vertex_buffers().push(buffer);
        vertex_buffer_layouts().push(vec![T::desc()]);

        self.to_owned()
    }

    pub fn with_indices(&mut self, indices: &[u32]) -> Self {
        self.index_buffer = index_buffers().len();
        self.num_indices = indices.len() as u32;

        index_buffers().push(
            device().create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
        );

        self.to_owned()
    }

    pub fn with_uniform<T: bytemuck::Pod + bytemuck::Zeroable + Clone + Copy>(
        &mut self,
        uniform: crate::uniform::Uniform<T>,
    ) -> Self {
        let bind_group_entry = uniform_bind_group_entry(self.uniform_bind_group_entry);
        let bind_group_layout_entry =
            uniform_bind_group_layout_entry(self.uniform_bind_group_layout_entry);

        let binding = bind_group_entry.len() as u32;

        bind_group_entry.push(wgpu::BindGroupEntry {
            binding,
            resource: uniform.buffer().as_entire_binding(),
        });

        bind_group_layout_entry.push(wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::all(),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });

        self.to_owned()
    }

    pub fn with_texture2d(&mut self, texture: crate::texture::Texture2D) -> Self {
        let bind_group_entry = texture_2d_bind_group_entry(self.texture_bind_group_entry);
        let bind_group_layout_entry =
            texture_2d_bind_group_layout_entry(self.texture_bind_group_layout_entry);

        let binding = bind_group_entry.len() as u32;

        bind_group_entry.push(wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::TextureView(texture.view()),
        });

        bind_group_layout_entry.push(wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        });

        self.to_owned()
    }

    pub fn init(&mut self) -> Self {
        let device = device();

        self.pipeline = render_pipelines().len();
        self.uniform_bind_group = uniform_bind_groups().len();
        self.texture_bind_group = texture_2d_bind_groups().len();

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: uniform_bind_group_layout_entry(self.uniform_bind_group_layout_entry),
            });

        uniform_bind_groups().push(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &uniform_bind_group_layout,
            entries: uniform_bind_group_entry(self.uniform_bind_group_entry),
        }));

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: texture_2d_bind_group_layout_entry(self.texture_bind_group_layout_entry),
            });

        texture_2d_bind_groups().push(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &texture_bind_group_layout,
            entries: texture_2d_bind_group_entry(self.texture_bind_group_entry),
        }));

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                sampler_2d_bind_group_layout(),
                &uniform_bind_group_layout,
                &texture_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        render_pipelines().push(
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: self.shader.module(),
                    entry_point: "vs_main",
                    buffers: vertex_buffer_layout(self.vertex_buffer_layout),
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                fragment: Some(wgpu::FragmentState {
                    module: self.shader.module(),
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: surface_config().format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview: None,
            }),
        );

        self.to_owned()
    }

    pub fn render(&self, draw_state: &mut FrameState) {
        let encoder = &mut draw_state.encoder;
        let frame = &draw_state.frame;
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: frame,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(render_pipeline(self.pipeline));

        render_pass.set_bind_group(0, sampler_2d_bind_group(), &[]);
        render_pass.set_bind_group(1, uniform_bind_group(self.texture_bind_group), &[]);
        render_pass.set_bind_group(2, texture_2d_bind_group(self.texture_bind_group), &[]);

        render_pass.set_vertex_buffer(0, vertex_buffer(self.vertex_buffer).slice(..));
        render_pass.set_index_buffer(
            index_buffer(self.index_buffer).slice(..),
            wgpu::IndexFormat::Uint32,
        );

        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}
