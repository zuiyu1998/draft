mod render_pass;

use crate::frame_graph::{
    PipelineContainer, ResourceRef, ResourceTable, ResourceView, TransientResource,
    TransientTextureView, TransientTextureViewDescriptor,
};
use draft_graphics::RenderDevice;
use wgpu::{CommandBuffer, CommandEncoder, CommandEncoderDescriptor, RenderPipeline};

pub use render_pass::*;

pub struct PassContext<'a> {
    device: &'a RenderDevice,
    command_encoder: CommandEncoder,
    resource_table: &'a ResourceTable,
    pipeline_container: &'a PipelineContainer,
}

impl PassContext<'_> {
    pub fn resource_table(&self) -> &ResourceTable {
        self.resource_table
    }

    pub fn device(&self) -> &RenderDevice {
        self.device
    }

    pub fn get_render_pipeline(&self, id: usize) -> &RenderPipeline {
        self.pipeline_container
            .get_render_pipeline(id)
            .expect("render pipeline mut have")
    }

    pub fn finish(self) -> CommandBuffer {
        self.command_encoder.finish()
    }

    pub fn get_resource<ResourceType: TransientResource, ViewType: ResourceView>(
        &self,
        resource_ref: &ResourceRef<ResourceType, ViewType>,
    ) -> &ResourceType {
        self.resource_table.get_resource(resource_ref)
    }

    pub fn create_texture_view_with_descriptor<ViewType: ResourceView>(
        &self,
        desc: &TransientTextureViewDescriptor<ViewType>,
    ) -> wgpu::TextureView {
        let resource = self.get_resource(&desc.texture);
        resource.resource.create_view(&desc.desc.get_desc())
    }

    pub fn create_texture_view(&self, texture_view: &TransientTextureView) -> wgpu::TextureView {
        match &texture_view {
            TransientTextureView::Read(desc) => self.create_texture_view_with_descriptor(desc),
            TransientTextureView::Write(desc) => self.create_texture_view_with_descriptor(desc),
            TransientTextureView::TextureView(texture_view) => texture_view.clone(),
        }
    }

    pub fn create_render_pass_color_attachment(
        &self,
        color_attachment: &TransientRenderPassColorAttachment,
    ) -> RenderPassColorAttachment {
        RenderPassColorAttachment {
            view: self.create_texture_view(&color_attachment.view),
            depth_slice: color_attachment.depth_slice,
            resolve_target: color_attachment
                .resolve_target
                .as_ref()
                .map(|resolve_target| self.create_texture_view(resolve_target)),
            ops: color_attachment.ops,
        }
    }

    pub fn create_render_pass_depth_stencil_attachment(
        &self,
        depth_stencil_attachment: &TransientRenderPassDepthStencilAttachment,
    ) -> RenderPassDepthStencilAttachment {
        RenderPassDepthStencilAttachment {
            view: self.create_texture_view(&depth_stencil_attachment.view),
            depth_ops: depth_stencil_attachment.depth_ops,
            stencil_ops: depth_stencil_attachment.stencil_ops,
        }
    }

    pub fn create_render_pass_descriptor(
        &self,
        desc: &TransientRenderPassDescriptor,
    ) -> RenderPassDescriptor {
        RenderPassDescriptor {
            label: desc.label.clone(),
            color_attachments: desc
                .color_attachments
                .iter()
                .map(|color_attachment| {
                    color_attachment.as_ref().map(|color_attachment| {
                        self.create_render_pass_color_attachment(color_attachment)
                    })
                })
                .collect(),
            depth_stencil_attachment: desc.depth_stencil_attachment.as_ref().map(
                |depth_stencil_attachment| {
                    self.create_render_pass_depth_stencil_attachment(depth_stencil_attachment)
                },
            ),
        }
    }
}

pub trait PassCommand: 'static + Send + Sync {
    fn execute(&self, context: &mut PassContext);
}

#[derive(Default)]
pub struct Pass {
    pub label: Option<String>,
    commands: Vec<Box<dyn PassCommand>>,
}

impl Pass {
    pub fn push<T: PassCommand>(&mut self, value: T) {
        self.commands.push(Box::new(value));
    }

    pub fn render(
        &self,
        command_buffers: &mut Vec<CommandBuffer>,
        device: &RenderDevice,
        resource_table: &ResourceTable,
        pipeline_container: &PipelineContainer,
    ) {
        let command_encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: self.label.as_deref(),
        });

        let mut pass_context = PassContext {
            device,
            command_encoder,
            resource_table,
            pipeline_container,
        };

        for command in self.commands.iter() {
            command.execute(&mut pass_context);
        }
        command_buffers.push(pass_context.finish());
    }
}
