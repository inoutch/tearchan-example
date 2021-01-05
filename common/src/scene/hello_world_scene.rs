use gfx_hal::command::{
    ClearColor, ClearDepthStencil, ClearValue, CommandBufferFlags, Level, SubpassContents,
};
use gfx_hal::image::{Extent, Layout};
use gfx_hal::pass::{Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, SubpassDesc};
use gfx_hal::pso::{PipelineStage, Rect};
use gfx_hal::queue::Submission;
use std::iter;
use std::iter::Once;
use tearchan::scene::context::{SceneContext, SceneRenderContext};
use tearchan::scene::factory::SceneFactory;
use tearchan::scene::{Scene, SceneControlFlow};
use tearchan_gfx::{CommandBuffer, Semaphore};
use winit::event::WindowEvent;

pub struct HelloWorldScene {}

impl HelloWorldScene {
    pub fn factory() -> SceneFactory {
        |_context, _| Box::new(HelloWorldScene {})
    }
}

impl Scene for HelloWorldScene {
    fn update(&mut self, _context: &mut SceneContext, _event: WindowEvent) -> SceneControlFlow {
        SceneControlFlow::None
    }

    fn render(&mut self, context: &mut SceneRenderContext) -> SceneControlFlow {
        let frame = context.gfx_rendering().frame();
        let gfx = context.gfx();
        let color_format = gfx.find_support_format();
        let depth_stencil_format = frame.depth_texture().format().clone();
        let extent = Extent {
            width: gfx.swapchain_desc().config.extent.width,
            height: gfx.swapchain_desc().config.extent.height,
            depth: 1,
        };
        let render_area = Rect {
            x: 0,
            y: 0,
            w: context.gfx().swapchain_desc().config.extent.width as _,
            h: context.gfx().swapchain_desc().config.extent.height as _,
        };

        frame
            .submission_complete_fence()
            .wait_for_fence(!0)
            .unwrap();
        frame.submission_complete_fence().reset_fence();

        let command_buffer = frame.command_pool().allocate_one(Level::Primary);
        command_buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);

        let render_pass = {
            let color_load_op = AttachmentLoadOp::Clear;
            let depth_load_op = AttachmentLoadOp::Clear;
            let attachment = Attachment {
                format: Some(color_format),
                samples: 1,
                ops: AttachmentOps::new(color_load_op, AttachmentStoreOp::Store),
                stencil_ops: AttachmentOps::DONT_CARE,
                layouts: Layout::Undefined..Layout::Present,
            };
            let depth_attachment = Attachment {
                format: Some(depth_stencil_format),
                samples: 1,
                ops: AttachmentOps::new(depth_load_op, AttachmentStoreOp::Store),
                stencil_ops: AttachmentOps::DONT_CARE,
                layouts: Layout::Undefined..Layout::DepthStencilAttachmentOptimal,
            };
            let subpass = SubpassDesc {
                colors: &[(0, Layout::ColorAttachmentOptimal)],
                depth_stencil: Some(&(1, Layout::DepthStencilAttachmentOptimal)),
                inputs: &[],
                resolves: &[],
                preserves: &[],
            };
            gfx.device()
                .create_render_pass(&[attachment, depth_attachment], &[subpass], &[])
        };
        let framebuffer = context.gfx().device().create_framebuffer_with_frame(
            &render_pass,
            &frame,
            vec![frame.depth_texture().image_view()],
            extent,
        );

        command_buffer.begin_render_pass(
            &render_pass,
            &framebuffer,
            render_area,
            &[
                ClearValue {
                    color: ClearColor {
                        float32: [0.3, 0.3, 0.3, 1.0],
                    },
                },
                ClearValue {
                    depth_stencil: ClearDepthStencil {
                        depth: 1.0f32,
                        stencil: 0,
                    },
                },
            ],
            SubpassContents::Inline,
        );
        command_buffer.end_render_pass();
        command_buffer.finish();
        let submission: Submission<
            Once<&CommandBuffer>,
            Vec<(&Semaphore, PipelineStage)>,
            Vec<&Semaphore>,
        > = Submission {
            command_buffers: iter::once(&command_buffer),
            wait_semaphores: vec![],
            signal_semaphores: vec![frame.submission_complete_semaphore()],
        };
        gfx.queue()
            .submit(submission, Some(frame.submission_complete_fence()));

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
