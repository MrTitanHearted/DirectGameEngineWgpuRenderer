use crate::common::*;

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, PartialOrd)]
pub struct Uniform<T: bytemuck::Pod + bytemuck::Zeroable + Clone + Copy> {
    buffer: usize,
    data: T,
}

impl<T: bytemuck::Pod + bytemuck::Zeroable + Clone + Copy> Uniform<T> {
    pub fn new(data: T) -> Self {
        use wgpu::util::DeviceExt;

        let device = device();

        let buffer = uniform_buffers().len();

        uniform_buffers().push(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[data]),
                usage: wgpu::BufferUsages::UNIFORM
                    | wgpu::BufferUsages::COPY_DST
                    | wgpu::BufferUsages::COPY_SRC,
            }),
        );

        Self { buffer, data }
    }

    pub fn get(&self) -> T {
        self.data
    }

    pub fn set(&mut self, data: T) {
        let queue = queue();

        self.data = data;
        let buffer = uniform_buffer(self.buffer);
        queue.write_buffer(buffer, 0, bytemuck::cast_slice(&[data]));
    }

    pub(crate) fn buffer(&self) -> &'static wgpu::Buffer {
        uniform_buffer(self.buffer)
    }
}
