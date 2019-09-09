use crate::lib::{graphics, util};

#[derive(Debug)]
pub struct QuadRenderer {
    vertex_buffer: wgpu::Buffer,
    vertex_count: usize,
    instance_buffer: wgpu::Buffer,
    instance_count: usize,
    max_instances: usize,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
}

impl QuadRenderer {
    pub fn new(graphics: &mut graphics::Graphics, max_instances: usize) -> Self {
        use std::mem;

        let sc_desc = &graphics.sc_desc;
        let device = &mut graphics.device;

        let mut init_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        let vertex_size = mem::size_of::<Vertex>();
        let vertex_data = create_quad();
        let vertex_buffer = device
            .create_buffer_mapped(vertex_data.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(&vertex_data);
        let vertex_count = vertex_data.len();
        let vb_desc = wgpu::VertexBufferDescriptor {
            stride: vertex_size  as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float2,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float2,
                    offset: 2 * mem::size_of::<f32>() as u64,
                    shader_location: 1,
                },
            ],
        };

        let instance_size = mem::size_of::<Instance>();
        let instance_buffer = device
            .create_buffer(&wgpu::BufferDescriptor {
                size: (instance_size * max_instances) as u64,
                usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
            });
        let ib_desc = wgpu::VertexBufferDescriptor {
            stride: vertex_size as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float2,
                    offset: 0,
                    shader_location: 2,
                },
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float2,
                    offset: 2 * mem::size_of::<f32>() as u64,
                    shader_location: 3,
                },
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float2,
                    offset: 4 * mem::size_of::<f32>() as u64,
                    shader_location: 4,
                },
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float3,
                    offset: 6 * mem::size_of::<f32>() as u64,
                    shader_location: 5,
                },
            ],
        };
        
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                    },
                },
                wgpu::BindGroupLayoutBinding {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        multisampled: false,
                        dimension: wgpu::TextureViewDimension::D2,
                    },
                },
                wgpu::BindGroupLayoutBinding {
                    binding: 2,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler,
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        use image::GenericImageView;
        let image = image::load_from_memory_with_format(include_bytes!("rust.png"), image::ImageFormat::PNG).unwrap();
        let image_dimensions = image.dimensions();
        let texture_data = image.as_rgba8().unwrap().to_vec();
        let texture_extent = wgpu::Extent3d {
            width: image_dimensions.0,
            height: image_dimensions.0,
            depth: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: texture_extent,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });
        let texture_view = texture.create_default_view();
        let temp_buffer = device.create_buffer_mapped(
            texture_data.len(), 
            wgpu::BufferUsage::COPY_SRC
        )
            .fill_from_slice(&texture_data);
        init_encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &temp_buffer,
                offset: 0,
                row_pitch: 4 * image_dimensions.0,
                image_height: image_dimensions.1,
            },
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                array_layer: 0,
                origin: wgpu::Origin3d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            texture_extent,
        );
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare_function: wgpu::CompareFunction::Always,
        });

        let global_uniform_size = mem::size_of::<GlobalUniforms>() as wgpu::BufferAddress;
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            size: global_uniform_size,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buffer,
                        range: 0..64,
                    },
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::Binding {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let vs_bytes = util::load_glsl(include_str!("shader.vert"), util::ShaderStage::Vertex);
        let fs_bytes = util::load_glsl(include_str!("shader.frag"), util::ShaderStage::Fragment);
        let vs_module = device.create_shader_module(&vs_bytes);
        let fs_module = device.create_shader_module(&fs_bytes);

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleStrip,
            color_states: &[
                wgpu::ColorStateDescriptor {
                    format: sc_desc.format,
                    color_blend: wgpu::BlendDescriptor {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    alpha_blend: wgpu::BlendDescriptor {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    write_mask: wgpu::ColorWrite::ALL,
                }
            ],
            depth_stencil_state: None,
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[vb_desc, ib_desc],
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        let init_command_buffer = init_encoder.finish();
        device.get_queue().submit(&[init_command_buffer]);

        Self {
            vertex_buffer,
            vertex_count,
            instance_buffer,
            instance_count: 0,
            max_instances,
            uniform_buffer,
            bind_group,
            render_pipeline,
        }
    }

    pub fn update(&mut self, graphics: &mut graphics::Graphics, instances: &[Instance]) {
        use std::cmp::min;
        let instances = &instances[0..min(self.max_instances, instances.len())];
        let num_instances = instances.len();
        println!("num_instances = {}", num_instances);
        if num_instances > 0 {
            self.instance_count = num_instances;
            let buffer_size = (num_instances * std::mem::size_of::<Instance>()) as u64;
            let temp_buffer = graphics.device
                .create_buffer_mapped(num_instances, wgpu::BufferUsage::COPY_SRC)
                .fill_from_slice(instances);

            let mut encoder = graphics.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
            encoder.copy_buffer_to_buffer(&temp_buffer, 0, &self.instance_buffer, 0, buffer_size);
            graphics.device.get_queue().submit(&[encoder.finish()]);
        }
    }

    pub fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        if self.instance_count > 0 {
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_vertex_buffers(0, &[(&self.vertex_buffer, 0), (&self.instance_buffer, 0)]);
            render_pass.draw(0..self.vertex_count as u32, 0..self.instance_count as u32);
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct GlobalUniforms {
    proj: cgmath::Matrix4<f32>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Instance {
    pub offset: [f32; 2],
    pub origin: [f32; 2],
    pub scale: [f32; 2],
    pub color: [f32; 3],
}

#[derive(Clone, Copy)]
struct Vertex {
    _pos: [f32; 2],
    _tex_coord: [f32; 2],
}

fn vertex(pos: [f32; 2], tex_coord: [f32; 2]) -> Vertex {
    Vertex { _pos: pos, _tex_coord: tex_coord } 
}

fn create_quad() -> Vec<Vertex> {
    let vertex_data = [
        vertex([-0.5, -0.5], [0.0, 0.0]),
        vertex([0.5, -0.5], [1.0, 0.0]),
        vertex([-0.5, 0.5], [0.0, 1.0]),
        vertex([0.5, 0.5], [1.0, 1.0]),
    ];
    vertex_data.to_vec()
}