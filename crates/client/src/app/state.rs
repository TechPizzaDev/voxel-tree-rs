use std::{
    collections::VecDeque,
    fs::File,
    io::Read,
    sync::Arc,
    time::{Duration, Instant},
};

use bytemuck::{Pod, Zeroable};
use egui_plot::{Plot, PlotPoint, PlotPoints};
use egui_wgpu::ScreenDescriptor;
use glam::{Mat4, Vec3, Vec4};
use rand::{SeedableRng, rngs::Xoshiro128PlusPlus};
use wgpu::{BindGroup, Buffer, BufferUsages, CommandEncoder, Extent3d, RenderPipeline};
use winit::{event::WindowEvent, window::Window};

use numerics::BinaryInteger;

use crate::{
    App, FrameIndex,
    app::point_cloud::{Point, PointGen},
    egui_tools::EguiRenderer,
    gpu::query::{QueryInfo, SubmitError},
};

#[derive(Clone, Copy, Default, Pod, Zeroable)]
#[repr(C)]
struct UniformData {
    mvp_matrix: Mat4,
    inverse_mvp_matrix: Mat4,
    time: Vec4,
    resolution: Vec4,
}

pub struct AppState {
    window_size: winit::dpi::PhysicalSize<u32>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    depth_texture: Option<wgpu::Texture>,
    surface_format: wgpu::TextureFormat,
    pub(crate) egui_renderer: EguiRenderer,
    scale_factor: f32,

    query_info: Option<QueryInfo>,
    uniform_buffer: Buffer,
    bind_group: BindGroup,
    raymarch_pipeline: RenderPipeline,
    point_buffer: Option<Buffer>,
    point_count: u32,
    point_pipeline: RenderPipeline,
    aspect_ratio: f32,
    pub(crate) current_frame: FrameIndex,
    main_pass_time_series: VecDeque<PlotPoint>,

    points_buf: Vec<Point>,
    colony: super::space_colony::SpaceColony,
    grow_time: Duration,

    // Drop order requires us to drop window last, or we segfault.
    window: Arc<Window>,
}

impl AppState {
    pub(crate) async fn new(
        app: &App,
        window: Arc<Window>,
        display: Box<dyn wgpu::wgt::WgpuHasDisplayHandle>,
    ) -> AppState {
        let mut instance_desc = wgpu::InstanceDescriptor::new_with_display_handle(display);
        instance_desc.flags = wgpu::InstanceFlags::debugging()
            .with_env()
            .union(wgpu::InstanceFlags::AUTOMATIC_TIMESTAMP_NORMALIZATION);
        let instance = wgpu::Instance::new(instance_desc);

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
        fn load_shader_source(app: &App, name: &str) -> String {
            let mut source = String::new();
            File::open(app.asset_dir.join("shaders").join(name))
                .expect("failed to open shader")
                .read_to_string(&mut source)
                .expect("failed to read shader");
            source
        }

        let raymarch_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(load_shader_source(app, "raymarch.wgsl").into()),
        });

        let point_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(load_shader_source(app, "point.wgsl").into()),
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
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });

        let raymarch_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &raymarch_shader,
                entry_point: Some("vertex_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &raymarch_shader,
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

        let point_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &point_shader,
                entry_point: Some("vertex_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: size_of::<Point>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![
                        0 => Float32x3,
                        1 => Float32,
                        2 => Uint32,
                    ],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &point_shader,
                entry_point: Some("fragment_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Max,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Less),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
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
            depth_texture: None,
            egui_renderer,
            scale_factor: 1.0,

            query_info,
            uniform_buffer,
            bind_group,
            raymarch_pipeline,
            point_buffer: None,
            point_count: 0,
            point_pipeline,

            aspect_ratio: 1.0,
            current_frame: 0.into(),
            main_pass_time_series: VecDeque::new(),

            points_buf: Vec::new(),
            colony: super::space_colony::SpaceColony::with_rng(
                20000,
                &mut Xoshiro128PlusPlus::seed_from_u64(0),
            ),
            grow_time: Duration::ZERO,
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

        self.depth_texture = Some(self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Buffer"),
            size: Extent3d {
                width: self.window_size.width,
                height: self.window_size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        }));
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

        let start = Instant::now();
        let grow_result = self.colony.grow();
        let grow_dur = Instant::now().duration_since(start);
        self.grow_time += grow_dur;

        println!(
            "{:?} = attractors: {}, nodes: {}, time: {:?}, total: {:?}",
            grow_result,
            self.colony.tree().attractors().len(),
            self.colony.tree().nodes().len(),
            grow_dur,
            self.grow_time
        );

        let proj_mat = Mat4::perspective_lh(
            (2.0 * std::f32::consts::PI) / 7.0,
            self.aspect_ratio,
            0.125,
            512.0,
        );

        let cam_radius = 350.;
        let (eye_z, eye_x) = (time * 0.33).sin_cos();

        let view_mat = Mat4::look_at_lh(
            Vec3::new(eye_x * cam_radius + 100., 250., eye_z * cam_radius + 100.),
            Vec3::new(100., 150., 100.),
            Vec3::new(0., 1.,0.))
        //    * Mat4::from_rotation_x(std::f32::consts::PI * 0.4)
        //    * Mat4::from_rotation_z(time * 0.25)
        //    * Mat4::from_rotation_y(time * 2.2 * 0.5);
        //     * Mat4::from_rotation_x(time * 0.1)
        ;

        let mvp_matrix = proj_mat * view_mat;
        let uniform_data = UniformData {
            mvp_matrix,
            inverse_mvp_matrix: mvp_matrix.inverse(),
            time: Vec4::zeroed().with_x(time),
            resolution: Vec4::new(
                self.window_size.width as f32,
                self.window_size.height as f32,
                0.,
                0.,
            ),
        };

        self.points_buf.clear();
        // TODO: egui
        if false {
            super::point_cloud::PointSphere {
                count: 1000,
                radius: 10.,
                point_size: 1.,
                point_color: crate::app::point_cloud::Rgba::rgb(0, 175, 255),
            }
            .generate(&mut self.points_buf);
        } else {
            self.colony.generate(&mut self.points_buf);
        }

        let point_bytes: &[u8] = bytemuck::cast_slice(&self.points_buf);
        if self
            .point_buffer
            .as_ref()
            .map(|buf| buf.size())
            .unwrap_or(0)
            < (point_bytes.len() as u64)
        {
            self.point_buffer = Some(self.device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: (point_bytes.len() as u64).round_up_to_pow_of_2(),
                usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));
        }
        self.point_count = self.points_buf.len() as u32;
        self.queue
            .write_buffer(self.point_buffer.as_ref().unwrap(), 0, point_bytes);

        self.queue
            .write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&uniform_data));

        let surface_texture = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(tex) => tex,
            wgpu::CurrentSurfaceTexture::Outdated => {
                // Ignoring outdated to allow resizing and minimization
                println!("wgpu surface outdated");
                return;
            }
            err => panic!("failed to acquire next swapchain texture: {:?}", err),
        };
        let surface_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                // Without add_srgb_suffix() the image we will be working with
                // might not be "gamma correct".
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });

        let depth_view = self
            .depth_texture
            .as_ref()
            .unwrap()
            .create_view(&wgpu::TextureViewDescriptor::default());

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
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        // If you wanted to call any drawing commands, they would go here.

        rpass.set_bind_group(0, &self.bind_group, &[]);

        if let Some(point_buffer) = &self.point_buffer {
            rpass.set_pipeline(&self.point_pipeline);
            rpass.set_vertex_buffer(0, point_buffer.slice(..));
            rpass.draw(0..6, 0..self.point_count);
        } else {
            rpass.set_pipeline(&self.raymarch_pipeline);
            rpass.draw(0..6, 0..1);
        }

        // End the renderpass.
        drop(rpass);

        while let Some(dur) = self.query_info.as_mut().and_then(Self::poll_frame_query) {
            self.main_pass_time_series.push_front(PlotPoint::new(
                dur.frame.0 as f64,
                dur.main_pass.as_secs_f64(),
            ));
        }
        // Submit after poll to reuse buffers more.
        self.update_frame_duration(&mut encoder);

        let mut gui_encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        let window = Arc::as_ref(&self.window);

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [self.window_size.width, self.window_size.height],
            pixels_per_point: window.scale_factor() as f32 * self.scale_factor,
        };

        {
            self.egui_renderer.begin_frame(window);
            let ctx = self.egui_renderer.context();

            egui::Window::new("Frame Duration")
                .default_open(false)
                .show(ctx, |ui| {
                    ui.label(format!(
                        "Window Size: {}x{}",
                        self.window_size.width, self.window_size.height
                    ));

                    ui.label("Main Pass:");
                    Plot::new("plot")
                        .allow_drag(false)
                        .allow_scroll(false)
                        .label_formatter(|name, point| {
                            if !name.is_empty() {
                                format!("{}: {:?}", name, Duration::from_secs_f64(point.y))
                            } else {
                                String::new()
                            }
                        })
                        .y_axis_formatter(|mark, _range| {
                            format!(
                                "{}{:?}",
                                mark.value
                                    .is_sign_negative()
                                    .then_some("-")
                                    .unwrap_or_default(),
                                Duration::from_secs_f64(mark.value.abs())
                            )
                        })
                        .show(ui, |plot| {
                            let (a, b) = self.main_pass_time_series.as_slices();
                            plot.line(egui_plot::Line::new("Main Pass", PlotPoints::from(a)));
                            plot.line(egui_plot::Line::new("Main Pass", PlotPoints::from(b)));
                        });
                });

            egui::Window::new("Control")
                .resizable(true)
                .vscroll(true)
                .default_open(false)
                .default_size(egui::Vec2::new(0., 0.))
                .show(ctx, |ui| {
                    if ui.button("Grow!").clicked() {
                        println!("{:?}", self.colony.grow());
                    }

                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(format!("Pixels per point: {}", ctx.pixels_per_point()));
                        if ui.button("-").clicked() {
                            self.scale_factor = (self.scale_factor - 0.1).max(0.3);
                        }
                        if ui.button("+").clicked() {
                            self.scale_factor = (self.scale_factor + 0.1).min(3.0);
                        }
                    });
                });

            self.egui_renderer.end_frame_and_draw(
                &self.device,
                &self.queue,
                &mut gui_encoder,
                window,
                &surface_view,
                screen_descriptor,
            );
        }

        self.queue.submit([encoder.finish(), gui_encoder.finish()]);

        self.window.pre_present_notify();
        surface_texture.present();
    }

    fn update_frame_duration(&mut self, encoder: &mut CommandEncoder) {
        let Some(qi) = &mut self.query_info else {
            return;
        };
        match qi.submit(encoder, self.current_frame) {
            Ok(()) => {}
            Err(SubmitError::BufferPoolEmpty) => {
                qi.add_buffer(&self.device);
                qi.submit(encoder, self.current_frame)
                    .expect("submit failed with new buffer");
            }
            Err(err) => unreachable!("{:?}", err),
        }
    }

    fn poll_frame_query(qi: &mut QueryInfo) -> Option<FrameDuration> {
        let mut bytes = Vec::new();
        match qi.poll(&mut bytes) {
            Ok(frame) => {
                let timestamp_start = u64::from_le_bytes(bytes[..8].try_into().unwrap());
                let timestamp_end = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
                let timestamp_delta = (timestamp_end - timestamp_start) as f32;

                let main_pass = Duration::from_nanos(timestamp_delta as u64);
                Some(FrameDuration { frame, main_pass })
            }
            Err(err) => {
                use crate::gpu::query::PollError;
                match err {
                    PollError::Empty => None,
                    PollError::BufferPool(frame) | PollError::BufferMap(frame) => {
                        println!("frame {}: dropped", frame);
                        None
                    }
                    PollError::ResultDisconnected => unreachable!(),
                }
            }
        }
    }
}

struct FrameDuration {
    frame: FrameIndex,
    main_pass: Duration,
}
