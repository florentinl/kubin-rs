use std::sync::Arc;

use solver::solvers::{methods::Methods, solver::Method};
use wgpu::SurfaceError;
use winit::{event::WindowEvent, keyboard::NamedKey, window::Window};

use crate::{
    cam::Camera,
    consts::{Direction, Faces},
    cube::Cube,
    depth_texture,
    vertex::Vertex,
};

pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,

    cube: Cube,
    camera: Camera,

    render_pipeline: wgpu::RenderPipeline,

    last_frame: std::time::Instant,
    window: Arc<Window>,
    depth_texture: depth_texture::DepthTexture,
    solver: Methods,
}
impl State {
    pub(crate) async fn new(window: Arc<Window>, solver: Methods) -> anyhow::Result<Self> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/main.wgsl"));
        let camera = Camera::new(size, &device);
        let depth_texture = depth_texture::DepthTexture::new(&device, size, "Depth Texture");

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&camera.camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth_texture::DepthTexture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // 1.
                stencil: wgpu::StencilState::default(),     // 2.
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let cube = Cube::new(&device);

        Ok(State {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            cube,
            camera,
            depth_texture,
            render_pipeline,
            last_frame: std::time::Instant::now(),
            window,
            solver,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
            self.depth_texture = depth_texture::DepthTexture::new(
                &self.device,
                self.window.inner_size(),
                "Depth Texture",
            );
            self.camera.update_aspect_ratio(width, height, &self.queue);
        }
    }

    pub fn update(&mut self) {
        let delta_time = self.last_frame.elapsed();
        self.last_frame += delta_time;

        self.camera.update(&self.queue);
        self.cube.update_cubies(delta_time, &self.queue);
    }

    pub(crate) fn render(&mut self) -> Result<(), SurfaceError> {
        self.window.request_redraw();

        if !self.is_surface_configured {
            return Ok(());
        }

        self.update();

        let surface_texture = self.surface.get_current_texture()?;
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera.camera_bind_group, &[]);

            for cubie in &self.cube.cubies {
                render_pass.set_vertex_buffer(0, cubie.vertex_buffer.slice(..));
                render_pass.draw(0..cubie.vertex_count, 0..1);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();

        Ok(())
    }

    pub(crate) fn input(&mut self, event: &WindowEvent) {
        self.camera.process_events(event);

        match event {
            WindowEvent::Resized(size) => {
                self.resize(size.width, size.height);
            }
            WindowEvent::RedrawRequested => {
                match self.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = self.window.inner_size();
                        self.resize(size.width, size.height);
                    }
                    Err(e) => {
                        log::error!("Unable to render {e}");
                    }
                }
            }
            WindowEvent::KeyboardInput { event, .. }
                if event.state == winit::event::ElementState::Pressed =>
            {
                let key = &event.logical_key;
                match key {
                    winit::keyboard::Key::Character(c) => {
                        let direction = if c.chars().next().unwrap().is_lowercase() {
                            Direction::Clockwise
                        } else {
                            Direction::CounterClockwise
                        };
                        let c = c.to_lowercase();

                        if c == "r" {
                            self.cube.start_animation(Faces::Right, direction);
                        } else if c == "l" {
                            self.cube.start_animation(Faces::Left, direction);
                        } else if c == "f" {
                            self.cube.start_animation(Faces::Front, direction);
                        } else if c == "b" {
                            self.cube.start_animation(Faces::Back, direction);
                        } else if c == "u" {
                            self.cube.start_animation(Faces::Up, direction);
                        } else if c == "d" {
                            self.cube.start_animation(Faces::Down, direction);
                        }
                    }
                    winit::keyboard::Key::Named(NamedKey::Enter) => {
                        let solution = self.solver.solve(&self.cube.algebric_representation);

                        log::info!("Solution: {solution:?}");

                        self.cube.start_algorithm(&solution);
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}
