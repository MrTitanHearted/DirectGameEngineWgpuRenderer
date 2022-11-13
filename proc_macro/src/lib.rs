use proc_macro::{TokenStream};

mod converter;
mod parser;
mod macros;

#[proc_macro_derive(VertexBufferLayout, attributes(
    u8x2, u8x4, s8x2, s8x4, un8x2, un8x4, sn8x2, sn8x4,
    u16x2, u16x4, s16x2, s16x4, un16x2, un16x4, sn16x2, sn16x4, f16x2, f16x4,
    f32, f32x2, f32x3, f32x4,
    u32, u32x2, u32x3, u32x4,
    s32, s32x2, s32x3, s32x4,
    f64, f64x2, f64x3, f64x4,
    mat2x2, mat2x3, mat2x4,
    mat3x2, mat3x3, mat3x4,
    mat4x2, mat4x3, mat4x4
))]
pub fn vertex_buffer_layout_derive(item: TokenStream) -> TokenStream { 
    macros::vertex_buffer_layout_derive(item, wgpu::VertexStepMode::Vertex)
}

#[proc_macro_derive(BytemuckDerive)]
pub fn bytemuck_derive(item: TokenStream) -> TokenStream {
    macros::bytemuck_derive(item)
}