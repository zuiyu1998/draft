pub mod render_pass;
pub mod render_pass_builder;

pub use render_pass::*;
pub use render_pass_builder::*;

use std::{borrow::Cow, mem::take};

use crate::{
    frame_graph::{EncoderCommand, EncoderCommandBuilder, FrameGraphContext, PassNodeBuilder},
    gfx_base::CommandEncoder,
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

impl EncoderCommandBuilder for PassBuilder<'_> {
    fn add_begin_encoder_command(&mut self, value: EncoderCommand) -> &mut Self {
        self.pass.begin_encoder_commands.push(value);

        self
    }

    fn add_end_encoder_command(&mut self, value: EncoderCommand) -> &mut Self {
        self.pass.end_encoder_commands.push(value);

        self
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

    pub fn add_executor<T: EncoderExecutor>(&mut self, executor: T) {
        self.pass.executors.push(Box::new(executor));
    }
}

#[derive(Default)]
pub struct Pass {
    pub label: Option<Cow<'static, str>>,
    begin_encoder_commands: Vec<EncoderCommand>,
    executors: Vec<Box<dyn EncoderExecutor>>,
    end_encoder_commands: Vec<EncoderCommand>,
}

impl Pass {
    pub fn render(&self, frame_graph_context: &mut FrameGraphContext) {
        let mut command_encoder = frame_graph_context
            .render_device
            .wgpu_device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: self.label.as_deref(),
            });

        for begin_encoder_command in self.begin_encoder_commands.iter() {
            begin_encoder_command.apply(&mut command_encoder);
        }

        for executor in self.executors.iter() {
            executor.execute(&mut command_encoder, frame_graph_context);
        }

        for end_encoder_command in self.end_encoder_commands.iter() {
            end_encoder_command.apply(&mut command_encoder);
        }

        let command_buffer = command_encoder.finish();

        frame_graph_context.add_command_buffer(command_buffer);
    }
}

pub trait EncoderExecutor: 'static + Send + Sync {
    fn execute(
        &self,
        command_encoder: &mut CommandEncoder,
        frame_graph_context: &mut FrameGraphContext,
    );
}
