use rand::Rng;
use std::{iter, mem};
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            }],
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.025, 0.0],
    },
    Vertex {
        position: [-0.0125, 0.0125],
    },
    Vertex {
        position: [-0.0125, -0.0125],
    },
];

const INDICES: &[u16] = &[0, 1, 2];
struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    window: Window,
    boid_manager: BoidManager,
    running: bool,
}

#[derive(Clone)]
pub struct Boid {
    position: [f32; 2],
    color: [f32; 3],
    rotation: f32, // Radians
    speed: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    pub offset: [f32; 2],
    pub color: [f32; 3],
    pub sin_cos: [f32; 2],
}

impl Instance {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Instance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub struct BoidManager {
    boids: Vec<Boid>,
}

impl BoidManager {
    pub fn new(count: usize) -> Self {
        let mut boids = Vec::with_capacity(count);

        let mut rng = rand::thread_rng();
        for _ in 0..count {
            boids.push(Boid {
                position: [
                    100.0 * rng.gen::<f32>() - 50.0,
                    100.0 * rng.gen::<f32>() - 50.0,
                ],
                color: [
                    rng.gen::<f32>() * 0.7,
                    rng.gen::<f32>() * 0.7,
                    rng.gen::<f32>() * 0.7,
                ],
                rotation: rng.gen::<f32>() * 2.0 * std::f32::consts::PI,
                speed: rng.gen::<f32>() * 2.0 + 1.0,
            });
        }

        Self { boids }
    }

    pub fn update(&mut self) {
        // let mut avg_rot = 0.0;
        // for boid in &mut self.boids {
        //     avg_rot += boid.rotation;
        // }
        // avg_rot /= self.boids.len() as f32;
        let old_boids = self.boids.clone();
        for boid in &mut self.boids {
            let mut speed = 0;
            let mut turn = 0;

            // boid.color = [0.0, 1.0, 0.0];
            for other in &old_boids {
                let dx = boid.position[0] - other.position[0];
                let dy = boid.position[1] - other.position[1];

                let dist = dx.powi(2) + dy.powi(2);
                if dist > 200.0 {
                    continue; // to far
                }

                let dot = boid.rotation.cos() * (-dx) + boid.rotation.sin() * (-dy);

                if dot < 0.3 {
                    continue; // not in fov
                }

                if boid.speed > other.speed {
                    speed -= 1;
                } else if boid.speed < other.speed {
                    speed += 1;
                }
                if boid.rotation > other.rotation {
                    turn -= 1;
                } else if boid.rotation < other.rotation {
                    turn += 1;
                }
                // boid.color = [0.5, 1.0, 0.5];
                if dist < 40.0 && dot > 0.7 {
                    // boid.color = [1.0, 0.0, 0.0];
                    if boid.rotation > other.rotation {
                        boid.rotation += 0.00000025 * (40.0 - dist).powi(3);
                    } else {
                        boid.rotation -= 0.00000025 * (40.0 - dist).powi(3);
                    }
                }
            }

            boid.speed += 0.00012 * speed.max(-1).min(1) as f32;
            boid.speed = boid.speed.max(1.0).min(3.0);

            boid.rotation += 0.00175 * turn.max(-1).min(1) as f32;
            // random turn
            boid.rotation += 0.0005 * (rand::thread_rng().gen::<f32>() - 0.5);

            boid.position[0] += boid.rotation.cos() * boid.speed / 300.0;
            boid.position[1] += boid.rotation.sin() * boid.speed / 300.0;

            boid.position[0] = ((boid.position[0] + 105.0).rem_euclid(210.0)) - 105.0;
            boid.position[1] = ((boid.position[1] + 105.0).rem_euclid(210.0)) - 105.0;
        }
    }

    pub fn into_instance_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        let content: Vec<Instance> = self
            .boids
            .iter()
            .map(|boid| Instance {
                offset: [boid.position[0] / 100.0, boid.position[1] / 100.0],
                color: boid.color,
                sin_cos: [boid.rotation.sin(), boid.rotation.cos()],
            })
            .collect();
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Boid Instance Buffer"),
            contents: bytemuck::cast_slice(&content),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    pub fn randomise_colour(&mut self) {
        let mut rng = rand::thread_rng();
        for boid in &mut self.boids {
            boid.color = [rng.gen(), rng.gen(), rng.gen()];
        }
    }

    pub fn randomise_position(&mut self) {
        let mut rng = rand::thread_rng();
        for boid in &mut self.boids {
            boid.position = [rng.gen(), rng.gen()];
        }
    }

    pub fn boids_len(&self) -> usize {
        self.boids.len()
    }
}

impl State {
    async fn new(window: Window) -> Self {
        let size = window.inner_size();
        window.set_title("Boids");
        let icon = include_bytes!("../assets/icon.png");
        let icon = image::load_from_memory(icon).unwrap();
        let icon = icon.to_rgba8();
        let (width, height) = icon.dimensions();
        let icon = winit::window::Icon::from_rgba(icon.into_raw(), width, height).unwrap();
        window.set_window_icon(Some(icon));

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc(), Instance::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                // or Features::POLYGON_MODE_POINT
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let boid_manager = BoidManager::new(250);
        let instance_buffer = boid_manager.into_instance_buffer(&device);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            instance_buffer,
            window,
            boid_manager,
            running: false,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {}

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.08,
                            g: 0.05,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            self.instance_buffer = self.boid_manager.into_instance_buffer(&self.device);

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(
                0..INDICES.len() as u32,
                0,
                0..self.boid_manager.boids_len() as u32,
            );
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // State::new uses async code, so we're going to wait for it to finish
    let mut state = State::new(window).await;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Space),
                                    ..
                                },
                            ..
                        } => {
                            state.running = !state.running;
                            // state.boid_manager.update();
                        }
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &mut so w have to dereference it twice
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        state.resize(state.size)
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // We're ignoring timeouts
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                state.window().request_redraw();
            }
            _ => {
                if state.running {
                    state.boid_manager.update()
                }
            }
        }
    });
}
