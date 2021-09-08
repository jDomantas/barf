use std::{
    cmp::Ordering,
    sync::Arc,
};
use wgpu::util::DeviceExt;
use crate::texture;
use crate::Texture as TextureIndex;

macro_rules! include_spirv {
    ($name:tt) => {{
        wgpu::include_spirv!(concat!(env!("OUT_DIR"), "/", $name))
    }};
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float2,
                }
            ],
        }
    }
}

const RENDERER_VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 1.0], tex_coords: [0.0, 1.0], },
    Vertex { position: [0.0, 0.0], tex_coords: [0.0, 0.0], },
    Vertex { position: [1.0, 0.0], tex_coords: [1.0, 0.0], },
    Vertex { position: [1.0, 1.0], tex_coords: [1.0, 1.0], },
];

const RENDERER_INDICES: &[u16] = &[
    0, 1, 2,
    0, 2, 3,
];

unsafe fn as_bytes<T>(slice: &[T]) -> &[u8] {
    let ptr = slice.as_ptr() as *const u8;
    let len = std::mem::size_of::<T>() * slice.len();
    std::slice::from_raw_parts(ptr, len)
}

pub(crate) struct Instance {
    pub(crate) texture: TextureIndex,
    pub(crate) raw: RawInstance,
}

#[derive(Clone, Copy)]
pub(crate) struct RawInstance {
    // fields are read by casting RawInstance to bytes and reading that
    #[allow(dead_code)]
    pub(crate) pos: [f32; 4],
    #[allow(dead_code)]
    pub(crate) size: [f32; 2],
    #[allow(dead_code)]
    pub(crate) tex_source_pos: [f32; 2],
    #[allow(dead_code)]
    pub(crate) tex_source_size_x: [f32; 2],
    #[allow(dead_code)]
    pub(crate) tex_source_size_y: [f32; 2],
    #[allow(dead_code)]
    pub(crate) tex_color: [f32; 4],
}

impl RawInstance {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<RawInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 10]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float4,
                },
            ],
        }
    }
}

fn cast_instances(instances: &[RawInstance]) -> &[u8] {
    unsafe { as_bytes(instances) }
}

pub(crate) struct WgpuState {
    surface: wgpu::Surface,
    pub(crate) device: Arc<wgpu::Device>,
    pub(crate) queue: Arc<wgpu::Queue>,
    pub(crate) texture_bind_group_layout: Arc<wgpu::BindGroupLayout>,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    pub(crate) size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    raw_instances: Vec<RawInstance>,
    instance_buffer: wgpu::Buffer,
    depth_texture: texture::Texture,
    pub(crate) textures: Vec<texture::BindableTexture>,
    pub(crate) instances: Vec<Instance>,
}

impl WgpuState {
    pub(crate) async fn new(window: &winit::window::Window) -> WgpuState {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ).await.unwrap();
        let device = Arc::new(device);
        let queue = Arc::new(queue);

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let texture_bind_group_layout = Arc::new(device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            comparison: false,
                            filtering: true,
                        },
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            }
        ));

        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: &[0; 100000],
                usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
            }
        );

        let pixel_texture = texture::Texture::from_rgba(
            &device,
            &queue,
            [255, 255, 255, 255].to_vec(),
            1,
            1,
            None,
            &[],
        );
        let pixel_texture = texture::BindableTexture::from_texture(
            pixel_texture,
            &device,
            &texture_bind_group_layout,
        );
        let depth_texture = texture::Texture::create_depth_texture(&device, &sc_desc, "depth_texture");

        let vs_module = device.create_shader_module(&include_spirv!("shader.vert.spv"));
        let fs_module = device.create_shader_module(&include_spirv!("shader.frag.spv"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &texture_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "main",
                buffers: &[
                    Vertex::desc(),
                    RawInstance::desc(),
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_module,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: sc_desc.format,
                    alpha_blend: wgpu::BlendState::REPLACE,
                    color_blend: wgpu::BlendState::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: wgpu::CullMode::Back,
                polygon_mode: wgpu::PolygonMode::Fill,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
                // Setting this to true requires Features::DEPTH_CLAMPING
                clamp_depth: false,
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: unsafe { as_bytes(RENDERER_VERTICES) },
                usage: wgpu::BufferUsage::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: unsafe { as_bytes(RENDERER_INDICES) },
                usage: wgpu::BufferUsage::INDEX,
            }
        );

        Self {
            surface,
            device,
            queue,
            texture_bind_group_layout,
            sc_desc,
            swap_chain,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            instances: Vec::new(),
            raw_instances: Vec::new(),
            instance_buffer,
            depth_texture,
            textures: vec![pixel_texture],
        }
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        if self.size.width != 0 && self.size.height != 0 {
            self.sc_desc.width = new_size.width;
            self.sc_desc.height = new_size.height;
            self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
            self.depth_texture = texture::Texture::create_depth_texture(&self.device, &self.sc_desc, "depth_texture");
        }
    }

    pub(crate) fn recreate_swap_chain(&mut self) {
        self.resize(self.size);
    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        if self.size.width == 0 || self.size.height == 0 {
            return Ok(());
        }

        let frame = self
            .swap_chain
            .get_current_frame()?
            .output;
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        self.instances.sort_by_key(|i| i.texture.index);
        self.raw_instances.clear();
        self.raw_instances.extend(self.instances.iter().map(|i| i.raw));
        self.queue.write_buffer(&self.instance_buffer, 0, cast_instances(&self.raw_instances));
        let mut idx = 0;
        while idx < self.instances.len() {
            let texture = self.instances[idx].texture;
            let count = match self.instances[idx..].binary_search_by(|item| {
                if item.texture.index <= texture.index {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }) {
                Ok(i) => i,
                Err(i) => i,
            };
            assert!(count > 0);
            let instance_range = (idx as u32)..((idx + count) as u32);
            idx += count;

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        }
                    }
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.textures[texture.index].bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..(RENDERER_INDICES.len() as u32), 0, instance_range);
        };
        self.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }
}
