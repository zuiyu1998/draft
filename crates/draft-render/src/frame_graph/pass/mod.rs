mod parameter;
mod render_pass;

pub use parameter::*;
pub use render_pass::*;

use std::{borrow::Cow, mem::take};

use crate::frame_graph::{FrameGraphContext, PassNodeBuilder, ResourceTable};

use draft_gfx_base::{
    CachedPipelineId, CommandEncoder, ComputePipeline, PipelineContainer, RawCommandBuffer,
    RenderDevice, RenderPipeline,
};

pub struct PassBuilder<'a> {
    pass_node_builder: PassNodeBuilder<'a>,
    pass: Pass,
}

impl Drop for PassBuilder<'_> {
    fn drop(&mut self) {
        let pass = take(&mut self.pass);
        self.pass_node_builder.set_pass(pass);
    }
}

impl<'a> PassBuilder<'a> {
    pub fn new(pass_node_builder: PassNodeBuilder<'a>) -> Self {
        PassBuilder {
            pass_node_builder,
            pass: Pass::default(),
        }
    }

    pub fn create_render_pass_builder<'b>(&'b mut self, name: &str) -> RenderPassBuilder<'a, 'b> {
        RenderPassBuilder::new(self, name)
    }

    pub fn push<T: PassCommand>(&mut self, command: T) {
        self.pass.commands.push(Box::new(command));
    }
}

#[derive(Default)]
pub struct Pass {
    pub label: Option<Cow<'static, str>>,
    commands: Vec<Box<dyn PassCommand>>,
}

impl Pass {
    pub fn render(&self, context: &mut FrameGraphContext) {
        let command_encoder = context.render_device.wgpu_device().create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: self.label.as_deref(),
            },
        );

        let mut pass_context = PassContext::new(
            &context.resource_table,
            &context.pipeline_container,
            &context.render_device,
            command_encoder,
        );

        for command in self.commands.iter() {
            command.execute(&mut pass_context);
        }

        context.add_command_buffer(pass_context.finish());
    }
}

pub trait PassCommand: 'static + Send + Sync {
    fn execute(&self, context: &mut PassContext);
}

pub struct PassContext<'a> {
    pub resource_table: &'a ResourceTable,
    pub pipeline_container: &'a PipelineContainer,
    pub render_device: &'a RenderDevice,
    pub command_encoder: CommandEncoder,
}

impl<'a> PassContext<'a> {
    pub fn new(
        resource_table: &'a ResourceTable,
        pipeline_container: &'a PipelineContainer,
        render_device: &'a RenderDevice,
        command_encoder: CommandEncoder,
    ) -> Self {
        Self {
            pipeline_container,
            resource_table,
            command_encoder,
            render_device,
        }
    }

    pub fn get_compute_pipeline(&self, id: CachedPipelineId) -> &ComputePipeline {
        self.pipeline_container
            .get_compute_pipeline(id)
            .expect("compute pipeline mut have")
    }

    pub fn get_render_pipeline(&self, id: CachedPipelineId) -> &RenderPipeline {
        self.pipeline_container
            .get_render_pipeline(id)
            .expect("render pipeline mut have")
    }

    pub fn finish(self) -> RawCommandBuffer {
        self.command_encoder.finish()
    }
}
