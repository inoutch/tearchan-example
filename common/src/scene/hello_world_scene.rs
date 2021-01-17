use nalgebra_glm::vec3;
use tearchan::scene::context::{SceneContext, SceneRenderContext};
use tearchan::scene::factory::SceneFactory;
use tearchan::scene::{Scene, SceneControlFlow};
use tearchan_gfx::camera::Camera3D;
use tearchan_util::math::rect::rect2;
use tearchan_util::mesh::square::{
    create_square_indices, create_square_positions, create_square_texcoords,
};
use wgpu::util::DeviceExt;
use winit::event::WindowEvent;

pub struct HelloWorldScene {
    index_count: usize,
    index_format: wgpu::IndexFormat,
    index_buffer: wgpu::Buffer,
    position_buffer: wgpu::Buffer,
    texcoord_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
    camera: Camera3D,
    angle: f32,
}

impl HelloWorldScene {
    pub fn factory() -> SceneFactory {
        |context, _| {
            let queue = context.gfx().queue;
            let device = context.gfx().device;
            let width = context.gfx().swapchain_desc.width as f32;
            let height = context.gfx().swapchain_desc.height as f32;
            let aspect = width / height;

            let indices = create_square_indices();
            let positions = create_square_positions(&rect2(0.0f32, 0.0f32, 1.0f32, 1.0f32))
                .iter()
                .map(|v| vec![v.x, v.y, v.z])
                .flatten()
                .collect::<Vec<_>>();
            let texcoords = create_square_texcoords(&rect2(0.0f32, 0.0f32, 1.0f32, 1.0f32))
                .iter()
                .map(|v| vec![v.x, v.y])
                .flatten()
                .collect::<Vec<_>>();

            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsage::INDEX,
            });
            let position_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Position Buffer"),
                contents: bytemuck::cast_slice(&positions),
                usage: wgpu::BufferUsage::VERTEX,
            });
            let texcoord_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Texcoord Buffer"),
                contents: bytemuck::cast_slice(&texcoords),
                usage: wgpu::BufferUsage::VERTEX,
            });

            let bind_group_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStage::VERTEX,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: wgpu::BufferSize::new(64),
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled: false,
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                view_dimension: wgpu::TextureViewDimension::D2,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::Sampler {
                                comparison: false,
                                filtering: true,
                            },
                            count: None,
                        },
                    ],
                });
            let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

            let size = 1u32;
            let texels = vec![255, 0, 0, 255];
            let texture_extent = wgpu::Extent3d {
                width: size,
                height: size,
                depth: 1,
            };
            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label: None,
                size: texture_extent,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            });
            let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            queue.write_texture(
                wgpu::TextureCopyView {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                },
                &texels,
                wgpu::TextureDataLayout {
                    offset: 0,
                    bytes_per_row: 4 * size,
                    rows_per_image: 0,
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
                ..Default::default()
            });
            let mut camera = Camera3D::default_with_aspect(aspect);
            camera.position = vec3(0.0f32, 2.0f32, 4.0f32);
            camera.target_position = vec3(0.0f32, 0.0f32, 0.0f32);
            camera.up = vec3(0.0f32, 1.0f32, 0.0f32);
            camera.update();

            let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(camera.combine().as_slice()),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            });

            // Create bind group
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
                label: None,
            });

            let index_format = wgpu::IndexFormat::Uint32;
            // Create the render pipeline
            let vertex_state = wgpu::VertexStateDescriptor {
                index_format: Some(index_format),
                vertex_buffers: &[
                    wgpu::VertexBufferDescriptor {
                        stride: 3 * std::mem::size_of::<f32>() as u64,
                        step_mode: wgpu::InputStepMode::Vertex,
                        attributes: &[wgpu::VertexAttributeDescriptor {
                            format: wgpu::VertexFormat::Float3,
                            offset: 0,
                            shader_location: 0,
                        }],
                    },
                    wgpu::VertexBufferDescriptor {
                        stride: 2 * std::mem::size_of::<f32>() as u64,
                        step_mode: wgpu::InputStepMode::Vertex,
                        attributes: &[wgpu::VertexAttributeDescriptor {
                            format: wgpu::VertexFormat::Float2,
                            offset: 0,
                            shader_location: 1,
                        }],
                    },
                ],
            };

            let vs_module = device.create_shader_module(&wgpu::include_spirv!(
                "../../../target/shaders/simple.vert.spv"
            ));
            let fs_module = device.create_shader_module(&wgpu::include_spirv!(
                "../../../target/shaders/simple.frag.spv"
            ));

            let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
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
                    cull_mode: wgpu::CullMode::Back,
                    ..Default::default()
                }),
                primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                color_states: &[wgpu::ColorStateDescriptor {
                    format: context.gfx().swapchain_desc.format,
                    color_blend: wgpu::BlendDescriptor::REPLACE,
                    alpha_blend: wgpu::BlendDescriptor::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
                depth_stencil_state: None,
                vertex_state: vertex_state.clone(),
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });

            Box::new(HelloWorldScene {
                index_format,
                index_count: indices.len(),
                index_buffer,
                position_buffer,
                texcoord_buffer,
                bind_group,
                uniform_buffer,
                pipeline,
                camera,
                angle: 0.0f32,
            })
        }
    }
}

impl Scene for HelloWorldScene {
    fn update(&mut self, _context: &mut SceneContext, _event: WindowEvent) -> SceneControlFlow {
        SceneControlFlow::None
    }

    fn render(&mut self, context: &mut SceneRenderContext) -> SceneControlFlow {
        let frame = context.gfx_rendering().frame();
        let queue = context.gfx().queue;
        let device = context.gfx().device;

        self.angle += 0.01f32;
        self.camera.position = vec3(self.angle.sin() * 4.0f32, 2.0f32, self.angle.cos() * 4.0f32);
        self.camera.target_position = vec3(0.0f32, 0.0f32, 0.0f32);
        self.camera.up = vec3(0.0f32, 1.0f32, 0.0f32);
        self.camera.update();
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(self.camera.combine().as_slice()),
        );

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
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
                    },
                }],
                depth_stencil_attachment: None,
            });
            rpass.push_debug_group("Prepare data for draw.");
            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.set_index_buffer(self.index_buffer.slice(..), self.index_format);
            rpass.set_vertex_buffer(0, self.position_buffer.slice(..));
            rpass.set_vertex_buffer(1, self.texcoord_buffer.slice(..));
            rpass.pop_debug_group();
            rpass.insert_debug_marker("Draw!");
            rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
        }

        queue.submit(Some(encoder.finish()));
        SceneControlFlow::None
    }
}

/*
impl HelloWorldScene {
    state: GameState,
    renderer: GameRenderer,
}

impl Scene for HelloWorldScene {
    fn update(&mut self, ctx: &mut Context) {
        let mut entity_command_buffer = ctx.entity_manager.create_command_buffer<GameStateComponent>();

        {
            let system_1_job = MainSystemJob::process(&self.state.actions, self.state.character.positions);
            let system_2_job = MainSystemJob::process(&self.state.actions, self.state.character.angles);
            let system_3_job = MainSystemJob::process(&self.state.input, self.state.character.positions);

            ctx.job_worker.barrier(&[system_1_job, system_1_job, system_3_job]);
        }

        // Operate component sync
        let mut component_factory = ctx.entity_manager.cmd(entity_command_buffer);
        while let command = component_factory.pull() {
            match command {
                ...
            }
        }

        {
            // Transform map for rendering
            let system_4_job = RenderBatchSystemJob::process(
                &self.state.character.positions,
                &self.state.character.angles,
                &self.state.character.status,
                &self.renderer.character,
            );

            // Transform characters for rendering
            let system_5_job = RenderBatchSystemJob::process(
                &self.state.character.positions,
                &self.state.character.angles,
                &self.state.character.status,
                &self.renderer.character,
            );

            ctx.job_worker.barrier(&[system_4_job, system_5_job]);
        }
    }

    fn render(&mut self, gfx: &mut Gfx) {
        self.renderer.render_all(gfx);
    }
}
*/
