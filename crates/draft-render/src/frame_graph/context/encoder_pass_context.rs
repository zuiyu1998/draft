use wgpu::{Extent3d, ImageSubresourceRange};

use crate::frame_graph::{
    Ref, ResourceRead, ResourceWrite, TexelCopyBufferInfo, TexelCopyTextureInfo, TransientBuffer,
    TransientTexture,
};

use super::{
    ClearBufferParameter, ClearTextureParameter, CopyTextureToBufferParameter,
    CopyTextureToTextureParameter, FrameGraphContext,
};

pub trait EncoderPassCommandBuilder {
    fn push_encoder_pass_command(&mut self, value: EncoderPassCommand);

    fn copy_texture_to_buffer(
        &mut self,
        source: TexelCopyTextureInfo<ResourceRead>,
        destination: TexelCopyBufferInfo<ResourceWrite>,
        copy_size: Extent3d,
    ) {
        self.push_encoder_pass_command(EncoderPassCommand::new(CopyTextureToBufferParameter {
            source,
            destination,
            copy_size,
        }));
    }

    fn clear_buffer(
        &mut self,
        buffer_ref: &Ref<TransientBuffer, ResourceWrite>,
        offset: u64,
        size: Option<u64>,
    ) {
        self.push_encoder_pass_command(EncoderPassCommand::new(ClearBufferParameter {
            buffer_ref: buffer_ref.clone(),
            offset,
            size,
        }));
    }

    fn clear_texture(
        &mut self,
        texture_ref: &Ref<TransientTexture, ResourceWrite>,
        subresource_range: ImageSubresourceRange,
    ) {
        self.push_encoder_pass_command(EncoderPassCommand::new(ClearTextureParameter {
            texture_ref: texture_ref.clone(),
            subresource_range,
        }));
    }

    fn copy_texture_to_texture(
        &mut self,
        source: TexelCopyTextureInfo<ResourceRead>,
        destination: TexelCopyTextureInfo<ResourceWrite>,
        copy_size: Extent3d,
    ) {
        self.push_encoder_pass_command(EncoderPassCommand::new(CopyTextureToTextureParameter {
            source,
            destination,
            copy_size,
        }));
    }
}

pub struct EncoderPassCommand(Box<dyn ErasedEncoderPassCommand>);

impl EncoderPassCommand {
    pub fn new<T: ErasedEncoderPassCommand>(value: T) -> Self {
        Self(Box::new(value))
    }

    pub fn execute(&self, command_encoder_context: &mut EncoderPassContext) {
        self.0.execute(command_encoder_context)
    }
}

pub trait ErasedEncoderPassCommand: Sync + Send + 'static {
    fn execute(&self, command_encoder_context: &mut EncoderPassContext);
}

pub struct EncoderPassContext<'a, 'b> {
    command_encoder: &'b mut wgpu::CommandEncoder,
    frame_graph_context: &'b mut FrameGraphContext<'a>,
}

impl<'a, 'b> EncoderPassContext<'a, 'b> {
    pub fn copy_texture_to_buffer(
        &mut self,
        source: TexelCopyTextureInfo<ResourceRead>,
        destination: TexelCopyBufferInfo<ResourceWrite>,
        copy_size: Extent3d,
    ) {
        let source_texture = self.frame_graph_context.get_resource(&source.texture);
        let destination_buffer = self.frame_graph_context.get_resource(&destination.buffer);

        self.command_encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfoBase {
                texture: &source_texture.resource,
                mip_level: source.mip_level,
                origin: source.origin,
                aspect: source.aspect,
            },
            wgpu::TexelCopyBufferInfoBase {
                buffer: &destination_buffer.resource,
                layout: destination.layout,
            },
            copy_size,
        );
    }

    pub fn clear_buffer(
        &mut self,
        buffer_ref: &Ref<TransientBuffer, ResourceWrite>,
        offset: u64,
        size: Option<u64>,
    ) {
        let buffer = self.frame_graph_context.get_resource(buffer_ref);

        self.command_encoder
            .clear_buffer(&buffer.resource, offset, size);
    }

    pub fn clear_texture(
        &mut self,
        texture_ref: &Ref<TransientTexture, ResourceWrite>,
        subresource_range: &ImageSubresourceRange,
    ) {
        let texture = self.frame_graph_context.get_resource(texture_ref);

        self.command_encoder
            .clear_texture(&texture.resource, subresource_range);
    }

    pub fn copy_texture_to_texture(
        &mut self,
        source: TexelCopyTextureInfo<ResourceRead>,
        destination: TexelCopyTextureInfo<ResourceWrite>,
        copy_size: Extent3d,
    ) {
        let source_texture = self.frame_graph_context.get_resource(&source.texture);
        let destination_texture = self.frame_graph_context.get_resource(&destination.texture);

        self.command_encoder.copy_texture_to_texture(
            wgpu::TexelCopyTextureInfoBase {
                texture: &source_texture.resource,
                mip_level: source.mip_level,
                origin: source.origin,
                aspect: source.aspect,
            },
            wgpu::TexelCopyTextureInfoBase {
                texture: &destination_texture.resource,
                mip_level: destination.mip_level,
                origin: destination.origin,
                aspect: destination.aspect,
            },
            copy_size,
        );
    }

    pub fn execute(mut self, commands: &[EncoderPassCommand]) {
        for command in commands {
            command.execute(&mut self);
        }
    }

    pub fn new(
        command_encoder: &'b mut wgpu::CommandEncoder,
        frame_graph_context: &'b mut FrameGraphContext<'a>,
    ) -> Self {
        EncoderPassContext {
            command_encoder,
            frame_graph_context,
        }
    }
}
