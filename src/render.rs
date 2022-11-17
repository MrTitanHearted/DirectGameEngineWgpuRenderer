use crate::{common::*, dstest::{init_depth_pass, mut_depth_pass}};

pub fn init_wgpu<
    T: raw_window_handle::HasRawWindowHandle + raw_window_handle::HasRawDisplayHandle,
>(
    backend: wgpu::Backends,
    window: &T,
    width: u32,
    height: u32,
) {
    let instance = wgpu::Instance::new(backend);
    let surface = unsafe { instance.create_surface(window) };
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
    }))
    .unwrap();
    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: Some("WGPU Device"),
            features: wgpu::Features::default(),
            limits: wgpu::Limits::default(),
        },
        None,
    ))
    .unwrap();
    let format = surface.get_supported_formats(&adapter)[0];
    let surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width,
        height,
        present_mode: wgpu::PresentMode::default(),
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
    };
    surface.configure(&device, &surface_config);

    let sampler_2d = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    let sampler_2d_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            }],
        });

    let sampler_2d_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Sampler(&sampler_2d),
        }],
        layout: &sampler_2d_bind_group_layout,
    });

    unsafe {
        INSTANCE = Some(instance);
        ADAPTER = Some(adapter);
        SURFACE = Some(surface);
        SURFACE_CONFIG = Some(surface_config);
        DEVICE = Some(device);
        QUEUE = Some(queue);
        SAMPLER_2D = Some(sampler_2d);
        SAMPLER_2D_BIND_GROUP = Some(sampler_2d_bind_group);
        SAMPLER_2D_BIND_GROUP_LAYOUT = Some(sampler_2d_bind_group_layout);
        init_depth_pass();
    }
}

pub fn resize_viewport(width: u32, height: u32) {
    if width > 0 && height > 0 {
        let surface = surface();
        let surface_config = surface_config();
        let device = device();
        surface_config.width = width;
        surface_config.height = height;
        surface.configure(device, surface_config);
        mut_depth_pass().resize();
    }
}

pub fn begin_frame(clear_color: Option<wgpu::Color>) -> FrameState {
    let current = surface().get_current_texture().unwrap();

    let frame = current.texture.create_view(&wgpu::TextureViewDescriptor {
        label: Some("WGPU Framebuffer"),
        ..Default::default()
    });

    let mut encoder = device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("WGPU Command Encoder"),
    });

    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("WGPU Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &frame,
            resolve_target: None,
            ops: wgpu::Operations {
                load: if let Some(clear_color) = clear_color {
                    wgpu::LoadOp::Clear(clear_color)
                } else {
                    wgpu::LoadOp::Load
                },
                store: true,
            },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: depth_stencil_texture_view(),
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: true
            }),
            stencil_ops: None,
        }),
    });

    FrameState::new(encoder, current, frame)
}

pub fn end_frame(draw_state: FrameState) {
    // depth_pass().render(&mut draw_state);

    let submission = draw_state.encoder.finish();
    queue().submit(std::iter::once(submission));
    draw_state.current_texture.present();
}
