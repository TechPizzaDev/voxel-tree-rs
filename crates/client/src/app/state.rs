use std::{borrow::Cow, sync::Arc};

use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3, Vec4};
use wgpu::{BindGroup, Buffer, BufferUsages, InstanceFlags, RenderPipeline, SurfaceError};
use winit::{event::WindowEvent, window::Window};

use crate::{app::FrameIndex, egui_tools::EguiRenderer, gpu::query::QueryInfo};

#[derive(Clone, Copy, Default, Pod, Zeroable)]
#[repr(C)]
struct UniformData {
    inverse_mvp_matrix: Mat4,
    time: Vec4,
}

pub struct AppState {
    window: Arc<Window>,
    window_size: winit::dpi::PhysicalSize<u32>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    pub(crate) egui_renderer: EguiRenderer,
    scale_factor: f32,

    query_info: Option<QueryInfo>,
    uniform_buffer: Buffer,
    bind_group: BindGroup,
    render_pipeline: RenderPipeline,
    aspect_ratio: f32,
    pub(crate) current_frame: FrameIndex,
}

impl AppState {
    pub(crate) async fn new(window: Arc<Window>) -> AppState {
        let mut instance_desc = wgpu::InstanceDescriptor::default();
        instance_desc.flags = InstanceFlags::debugging()
            .with_env()
            .union(InstanceFlags::AUTOMATIC_TIMESTAMP_NORMALIZATION);
        let instance = wgpu::Instance::new(&instance_desc);

        let mut adapter_desc = wgpu::RequestAdapterOptions::default();
        adapter_desc.power_preference = wgpu::PowerPreference::HighPerformance;
        let adapter = instance.request_adapter(&adapter_desc).await.unwrap();

        let optional_features =
            wgpu::Features::TIMESTAMP_QUERY | wgpu::Features::TIMESTAMP_QUERY_INSIDE_PASSES;
        let cross_features = adapter.features().intersection(optional_features);
        if !cross_features.is_empty() {
            println!("requesting optional wgpu features: {}", cross_features);
        }

        let mut device_desc = wgpu::DeviceDescriptor::default();
        device_desc.required_features.insert(cross_features);
        let (device, queue) = adapter.request_device(&device_desc).await.unwrap();
        assert_eq!(queue.get_timestamp_period(), 1.0);

        let window_size = window.inner_size();

        let surface = instance.create_surface(window.clone()).unwrap();
        let caps = surface.get_capabilities(&adapter);
        let surface_format = caps.formats[0];

        // Load the shaders from disk
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: size_of::<UniformData>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            immediate_size: 0,
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment_main"),
                compilation_options: Default::default(),
                targets: &[Some(surface_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let query_info = device
            .features()
            .contains(wgpu::Features::TIMESTAMP_QUERY_INSIDE_PASSES)
            .then(|| QueryInfo::new(0..2, &device));

        let egui_renderer = EguiRenderer::new(&device, surface_format, None, 1, &window);

        let mut state = AppState {
            window,
            window_size,
            device,
            queue,
            surface,
            surface_format,
            egui_renderer,
            scale_factor: 1.0,

            query_info,
            uniform_buffer,
            bind_group,
            render_pipeline,
            aspect_ratio: 1.0,
            current_frame: 0.into(),
        };

        // Configure surface for the first time
        state.configure_surface();

        state
    }

    pub(crate) fn get_window(&self) -> &Window {
        &self.window
    }

    fn configure_surface(&mut self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            // Request compatibility with the sRGB-format texture view we‘re going to create later.
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.window_size.width,
            height: self.window_size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
        self.aspect_ratio = self.window_size.width as f32 / self.window_size.height as f32;
    }

    pub(crate) fn handle_event(&mut self, event: &WindowEvent) {
        // let egui render to process the event first
        self.egui_renderer.handle_input(&self.window, event);
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.window_size = new_size;

        self.configure_surface();
    }

    pub(crate) fn render(&mut self) {
        let time = self.current_frame.0 as f32 / 60.0;

        let proj_mat = Mat4::perspective_lh(
            (2.0 * std::f32::consts::PI) / 7.0,
            self.aspect_ratio,
            1.0,
            16.0,
        );

        let view_mat = Mat4::from_translation(Vec3::new(0., 0., 24.))
            * Mat4::from_rotation_x(std::f32::consts::PI * 0.4)
            * Mat4::from_rotation_z(time * 0.25);

        let data = UniformData {
            inverse_mvp_matrix: (proj_mat * view_mat).inverse(),
            time: Vec4::zeroed().with_x(time),
        };

        self.queue
            .write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&data));

        let surface_texture = match self.surface.get_current_texture() {
            Ok(tex) => tex,
            Err(SurfaceError::Outdated) => {
                // Ignoring outdated to allow resizing and minimization
                println!("wgpu surface outdated");
                return;
            }
            Err(err) => panic!("failed to acquire next swapchain texture: {}", err),
        };
        let surface_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                // Without add_srgb_suffix() the image we will be working with
                // might not be "gamma correct".
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });

        let mut encoder = self.device.create_command_encoder(&Default::default());

        let timestamp_writes = self
            .query_info
            .as_ref()
            .map(|q| wgpu::RenderPassTimestampWrites {
                query_set: q.query_set(),
                beginning_of_pass_write_index: Some(0),
                end_of_pass_write_index: Some(1),
            });

        // Create the renderpass which will clear the screen.
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &surface_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 100.0 / 255.0,
                        g: 149.0 / 255.0,
                        b: 237.0 / 255.0,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        // If you wanted to call any drawing commands, they would go here.

        rpass.set_bind_group(0, &self.bind_group, &[]);
        rpass.set_pipeline(&self.render_pipeline);
        rpass.draw(0..6, 0..1);

        // End the renderpass.
        drop(rpass);

        self.queue.submit([encoder.finish()]);

        self.window.pre_present_notify();
        surface_texture.present();
    }
}
