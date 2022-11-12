pub(crate) static mut INSTANCE: Option<wgpu::Instance> = None;
pub(crate) static mut ADAPTER: Option<wgpu::Adapter> = None;
pub(crate) static mut SURFACE: Option<wgpu::Surface> = None;
pub(crate) static mut SURFACE_CONFIG: Option<wgpu::SurfaceConfiguration> = None;
pub(crate) static mut DEVICE: Option<wgpu::Device> = None;
pub(crate) static mut QUEUE: Option<wgpu::Queue> = None;

pub(crate) static mut SAMPLER_2D: Option<wgpu::Sampler> = None;
pub(crate) static mut SAMPLER_2D_BIND_GROUP: Option<wgpu::BindGroup> = None;
pub(crate) static mut SAMPLER_2D_BIND_GROUP_LAYOUT: Option<wgpu::BindGroupLayout> = None;

pub(crate) static mut VERTEX_BUFFERS: Vec<wgpu::Buffer> = Vec::new();
pub(crate) static mut INDEX_BUFFERS: Vec<wgpu::Buffer> = Vec::new();

pub(crate) static mut SHADERS: Vec<wgpu::ShaderModule> = Vec::new();

pub(crate) static mut VERTEX_BUFFER_LAYOUTS: Vec<Vec<wgpu::VertexBufferLayout<'static>>> =
    Vec::new();

pub(crate) static mut UNIFORM_BUFFERS: Vec<wgpu::Buffer> = Vec::new();
pub(crate) static mut UNIFORM_BIND_GROUP: Vec<wgpu::BindGroup> = Vec::new();
pub(crate) static mut UNIFORM_BIND_GROUP_ENTRIES: Vec<Vec<wgpu::BindGroupEntry<'static>>> =
    Vec::new();
pub(crate) static mut UNIFORM_BIND_GROUP_LAYOUT_ENTRIES: Vec<Vec<wgpu::BindGroupLayoutEntry>> =
    Vec::new();

pub(crate) static mut TEXTURE_VIEWS: Vec<wgpu::TextureView> = Vec::new();
pub(crate) static mut TEXTURE_2D_BIND_GROUP: Vec<wgpu::BindGroup> = Vec::new();
pub(crate) static mut TEXTURE_2D_BIND_GROUP_ENTRIES: Vec<Vec<wgpu::BindGroupEntry<'static>>> =
    Vec::new();
pub(crate) static mut TEXTURE_2D_BIND_GROUP_LAYOUT_ENTRIES: Vec<Vec<wgpu::BindGroupLayoutEntry>> =
    Vec::new();

pub(crate) static mut RENDER_PIPELINES: Vec<wgpu::RenderPipeline> = Vec::new();

pub(crate) fn surface() -> &'static wgpu::Surface {
    unsafe { SURFACE.as_ref().unwrap() }
}

pub(crate) fn surface_config() -> &'static mut wgpu::SurfaceConfiguration {
    unsafe { SURFACE_CONFIG.as_mut().unwrap() }
}

pub(crate) fn device() -> &'static wgpu::Device {
    unsafe { DEVICE.as_ref().unwrap() }
}

pub(crate) fn queue() -> &'static wgpu::Queue {
    unsafe { QUEUE.as_ref().unwrap() }
}

pub(crate) fn sampler_2d_bind_group() -> &'static wgpu::BindGroup {
    unsafe { SAMPLER_2D_BIND_GROUP.as_ref().unwrap() }
}

pub(crate) fn sampler_2d_bind_group_layout() -> &'static wgpu::BindGroupLayout {
    unsafe { SAMPLER_2D_BIND_GROUP_LAYOUT.as_ref().unwrap() }
}

pub(crate) fn vertex_buffers() -> &'static mut Vec<wgpu::Buffer> {
    unsafe { VERTEX_BUFFERS.as_mut() }
}

pub(crate) fn vertex_buffer(id: crate::buffer::VertexBuffer) -> &'static wgpu::Buffer {
    unsafe { &VERTEX_BUFFERS[id] }
}

pub(crate) fn index_buffers() -> &'static mut Vec<wgpu::Buffer> {
    unsafe { INDEX_BUFFERS.as_mut() }
}

pub(crate) fn index_buffer(id: crate::buffer::IndexBuffer) -> &'static wgpu::Buffer {
    unsafe { &INDEX_BUFFERS[id] }
}

pub(crate) fn vertex_buffer_layouts() -> &'static mut Vec<Vec<wgpu::VertexBufferLayout<'static>>> {
    unsafe { VERTEX_BUFFER_LAYOUTS.as_mut() }
}

pub(crate) fn vertex_buffer_layout(
    id: crate::buffer::VertexBufferLayout,
) -> &'static Vec<wgpu::VertexBufferLayout<'static>> {
    unsafe { &VERTEX_BUFFER_LAYOUTS[id] }
}

pub(crate) fn render_pipelines() -> &'static mut Vec<wgpu::RenderPipeline> {
    unsafe { RENDER_PIPELINES.as_mut() }
}

pub(crate) fn render_pipeline(id: crate::buffer::RenderPipeline) -> &'static wgpu::RenderPipeline {
    unsafe { &RENDER_PIPELINES[id] }
}

pub(crate) fn uniform_buffers() -> &'static mut Vec<wgpu::Buffer> {
    unsafe { UNIFORM_BUFFERS.as_mut() }
}

pub(crate) fn uniform_buffer(id: usize) -> &'static wgpu::Buffer {
    unsafe { &UNIFORM_BUFFERS[id] }
}

pub(crate) fn uniform_bind_groups() -> &'static mut Vec<wgpu::BindGroup> {
    unsafe { UNIFORM_BIND_GROUP.as_mut() }
}

pub(crate) fn uniform_bind_group(id: usize) -> &'static wgpu::BindGroup {
    unsafe { &UNIFORM_BIND_GROUP[id] }
}

pub(crate) fn uniform_bind_group_entries() -> &'static mut Vec<Vec<wgpu::BindGroupEntry<'static>>> {
    unsafe { UNIFORM_BIND_GROUP_ENTRIES.as_mut() }
}

pub(crate) fn uniform_bind_group_entry(
    id: usize,
) -> &'static mut Vec<wgpu::BindGroupEntry<'static>> {
    unsafe { UNIFORM_BIND_GROUP_ENTRIES[id].as_mut() }
}

pub(crate) fn uniform_bind_group_layout_entries(
) -> &'static mut Vec<Vec<wgpu::BindGroupLayoutEntry>> {
    unsafe { UNIFORM_BIND_GROUP_LAYOUT_ENTRIES.as_mut() }
}

pub(crate) fn uniform_bind_group_layout_entry(
    id: usize,
) -> &'static mut Vec<wgpu::BindGroupLayoutEntry> {
    unsafe { UNIFORM_BIND_GROUP_LAYOUT_ENTRIES[id].as_mut() }
}

pub(crate) fn texture_views() -> &'static mut Vec<wgpu::TextureView> {
    unsafe { TEXTURE_VIEWS.as_mut() }
}

pub(crate) fn texture_view(id: usize) -> &'static wgpu::TextureView {
    unsafe { &TEXTURE_VIEWS[id] }
}

pub(crate) fn texture_2d_bind_groups() -> &'static mut Vec<wgpu::BindGroup> {
    unsafe { TEXTURE_2D_BIND_GROUP.as_mut() }
}

pub(crate) fn texture_2d_bind_group(id: usize) -> &'static wgpu::BindGroup {
    unsafe { &TEXTURE_2D_BIND_GROUP[id] }
}

pub(crate) fn texture_2d_bind_group_entries() -> &'static mut Vec<Vec<wgpu::BindGroupEntry<'static>>>
{
    unsafe { TEXTURE_2D_BIND_GROUP_ENTRIES.as_mut() }
}

pub(crate) fn texture_2d_bind_group_entry(
    id: usize,
) -> &'static mut Vec<wgpu::BindGroupEntry<'static>> {
    unsafe { TEXTURE_2D_BIND_GROUP_ENTRIES[id].as_mut() }
}

pub(crate) fn texture_2d_bind_group_layout_entries(
) -> &'static mut Vec<Vec<wgpu::BindGroupLayoutEntry>> {
    unsafe { TEXTURE_2D_BIND_GROUP_LAYOUT_ENTRIES.as_mut() }
}

pub(crate) fn texture_2d_bind_group_layout_entry(
    id: usize,
) -> &'static mut Vec<wgpu::BindGroupLayoutEntry> {
    unsafe { &mut TEXTURE_2D_BIND_GROUP_LAYOUT_ENTRIES[id] }
}

pub trait VertexBufferLayout {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

#[derive(Debug)]
pub struct FrameState {
    pub(crate) encoder: wgpu::CommandEncoder,
    pub(crate) current_texture: wgpu::SurfaceTexture,
    pub(crate) frame: wgpu::TextureView,
}

impl FrameState {
    pub(crate) fn new(
        encoder: wgpu::CommandEncoder,
        current_texture: wgpu::SurfaceTexture,
        frame: wgpu::TextureView,
    ) -> Self {
        Self {
            encoder,
            current_texture,
            frame,
        }
    }
}
