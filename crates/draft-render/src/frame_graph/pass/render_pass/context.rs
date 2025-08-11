use wgpu::{Extent3d, ImageSubresourceRange, QuerySet, ShaderStages};

use crate::{
    frame_graph::{
        BindGroup, BindGroupInfo, GpuRenderPass, PassContext, Ref, ResourceRead, ResourceWrite,
        TexelCopyBufferInfo, TexelCopyTextureInfo, TransientBuffer, TransientTexture,
    },
    gfx_base::CachedPipelineId,
};

use super::super::{
    BeginPipelineStatisticsQueryParameter, ClearBufferParameter, ClearTextureParameter,
    CopyTextureToBufferParameter, CopyTextureToTextureParameter, DrawIndexedIndirectParameter,
    DrawIndexedParameter, DrawIndirectParameter, DrawParameter,
    EndPipelineStatisticsQueryParameter, InsertDebugMarkerParameter,
    MultiDrawIndexedIndirectCountParameter, MultiDrawIndexedIndirectParameter,
    MultiDrawIndirectParameter, PopDebugGroupParameter, PushDebugGroupParameter,
    SetBindGroupParameter, SetBlendConstantParameter, SetIndexBufferParameter,
    SetPushConstantsParameter, SetRenderPipelineParameter, SetScissorRectParameter,
    SetStencilReferenceParameter, SetVertexBufferParameter, SetViewportParameter,
    WriteTimestampParameter,
};

use core::ops::Range;

pub trait RenderPassCommandBuilder {
    fn push_render_pass_command<T: RenderPassCommand>(&mut self, value: T);

    fn copy_texture_to_buffer(
        &mut self,
        source: TexelCopyTextureInfo<ResourceRead>,
        destination: TexelCopyBufferInfo<ResourceWrite>,
        copy_size: Extent3d,
    ) {
        self.push_render_pass_command(CopyTextureToBufferParameter {
            source,
            destination,
            copy_size,
        });
    }

    fn clear_buffer(
        &mut self,
        buffer_ref: &Ref<TransientBuffer, ResourceWrite>,
        offset: u64,
        size: Option<u64>,
    ) {
        self.push_render_pass_command(ClearBufferParameter {
            buffer_ref: buffer_ref.clone(),
            offset,
            size,
        });
    }

    fn clear_texture(
        &mut self,
        texture_ref: &Ref<TransientTexture, ResourceWrite>,
        subresource_range: ImageSubresourceRange,
    ) {
        self.push_render_pass_command(ClearTextureParameter {
            texture_ref: texture_ref.clone(),
            subresource_range,
        });
    }

    fn copy_texture_to_texture(
        &mut self,
        source: TexelCopyTextureInfo<ResourceRead>,
        destination: TexelCopyTextureInfo<ResourceWrite>,
        copy_size: Extent3d,
    ) {
        self.push_render_pass_command(CopyTextureToTextureParameter {
            source,
            destination,
            copy_size,
        });
    }

    fn draw_indirect(
        &mut self,
        indirect_buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        indirect_offset: u64,
    ) {
        self.push_render_pass_command(DrawIndirectParameter {
            indirect_buffer_ref: indirect_buffer_ref.clone(),
            indirect_offset,
        });
    }

    fn draw_indexed_indirect(
        &mut self,
        indirect_buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        indirect_offset: u64,
    ) {
        self.push_render_pass_command(DrawIndexedIndirectParameter {
            indirect_buffer_ref: indirect_buffer_ref.clone(),
            indirect_offset,
        });
    }

    fn multi_draw_indirect(
        &mut self,
        indirect_buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        indirect_offset: u64,
        count: u32,
    ) {
        self.push_render_pass_command(MultiDrawIndirectParameter {
            indirect_buffer_ref: indirect_buffer_ref.clone(),
            indirect_offset,
            count,
        });
    }

    fn multi_draw_indirect_count(
        &mut self,
        indirect_buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        indirect_offset: u64,
        count_buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        count_offset: u64,
        max_count: u32,
    ) {
        self.push_render_pass_command(MultiDrawIndexedIndirectCountParameter {
            indirect_buffer_ref: indirect_buffer_ref.clone(),
            indirect_offset,
            count_buffer_ref: count_buffer_ref.clone(),
            count_offset,
            max_count,
        });
    }

    fn multi_draw_indexed_indirect(
        &mut self,
        indirect_buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        indirect_offset: u64,
        count: u32,
    ) {
        self.push_render_pass_command(MultiDrawIndexedIndirectParameter {
            indirect_buffer_ref: indirect_buffer_ref.clone(),
            indirect_offset,
            count,
        });
    }

    fn multi_draw_indexed_indirect_count(
        &mut self,
        indirect_buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        indirect_offset: u64,
        count_buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        count_offset: u64,
        max_count: u32,
    ) {
        self.push_render_pass_command(MultiDrawIndexedIndirectCountParameter {
            indirect_buffer_ref: indirect_buffer_ref.clone(),
            indirect_offset,
            count_buffer_ref: count_buffer_ref.clone(),
            count_offset,
            max_count,
        });
    }

    fn set_stencil_reference(&mut self, reference: u32) {
        self.push_render_pass_command(SetStencilReferenceParameter { reference });
    }

    fn set_push_constants(&mut self, stages: ShaderStages, offset: u32, data: &[u8]) {
        self.push_render_pass_command(SetPushConstantsParameter {
            stages,
            offset,
            data: data.to_vec(),
        });
    }

    fn set_viewport(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
    ) {
        self.push_render_pass_command(SetViewportParameter {
            x,
            y,
            width,
            height,
            min_depth,
            max_depth,
        });
    }

    fn insert_debug_marker(&mut self, label: &str) {
        self.push_render_pass_command(InsertDebugMarkerParameter {
            label: label.to_string(),
        });
    }

    fn push_debug_group(&mut self, label: &str) {
        self.push_render_pass_command(PushDebugGroupParameter {
            label: label.to_string(),
        });
    }

    fn pop_debug_group(&mut self) {
        self.push_render_pass_command(PopDebugGroupParameter);
    }

    fn set_blend_constant(&mut self, color: wgpu::Color) {
        self.push_render_pass_command(SetBlendConstantParameter { color });
    }

    fn write_timestamp(&mut self, query_set: &QuerySet, index: u32) {
        self.push_render_pass_command(WriteTimestampParameter {
            query_set: query_set.clone(),
            index,
        });
    }

    fn begin_pipeline_statistics_query(&mut self, query_set: &QuerySet, index: u32) {
        self.push_render_pass_command(BeginPipelineStatisticsQueryParameter {
            query_set: query_set.clone(),
            index,
        });
    }

    fn end_pipeline_statistics_query(&mut self) {
        self.push_render_pass_command(EndPipelineStatisticsQueryParameter);
    }

    fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        self.push_render_pass_command(DrawIndexedParameter {
            indices,
            base_vertex,
            instances,
        });
    }

    fn set_scissor_rect(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.push_render_pass_command(SetScissorRectParameter {
            x,
            y,
            width,
            height,
        });
    }

    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.push_render_pass_command(DrawParameter {
            vertices,
            instances,
        });
    }

    fn set_index_buffer(
        &mut self,
        buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        index_format: wgpu::IndexFormat,
        offset: u64,
        size: u64,
    ) {
        self.push_render_pass_command(SetIndexBufferParameter {
            buffer_ref: buffer_ref.clone(),
            index_format,
            offset,
            size,
        });
    }

    fn set_render_pipeline(&mut self, id: CachedPipelineId) {
        self.push_render_pass_command(SetRenderPipelineParameter { id });
    }

    fn set_bind_group(&mut self, index: u32, bind_group: &BindGroupInfo, offsets: &[u32]) {
        self.push_render_pass_command(SetBindGroupParameter {
            index,
            bind_group: bind_group.clone(),
            offsets: offsets.to_vec(),
        });
    }

    fn set_vertex_buffer(
        &mut self,
        slot: u32,
        buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        offset: u64,
        size: u64,
    ) {
        self.push_render_pass_command(SetVertexBufferParameter {
            slot,
            buffer_ref: buffer_ref.clone(),
            offset,
            size,
        });
    }
}

pub trait RenderPassCommand: Sync + Send + 'static {
    fn execute(&self, render_pass_context: &mut RenderPassContext);
}

pub struct RenderPassContext<'a, 'b> {
    render_pass: GpuRenderPass,
    pass_context: &'b mut PassContext<'a>,
}

impl<'a, 'b> RenderPassContext<'a, 'b> {
    pub fn new(render_pass: GpuRenderPass, pass_context: &'b mut PassContext<'a>) -> Self {
        RenderPassContext {
            render_pass,
            pass_context,
        }
    }

    pub fn copy_texture_to_buffer(
        &mut self,
        source: TexelCopyTextureInfo<ResourceRead>,
        destination: TexelCopyBufferInfo<ResourceWrite>,
        copy_size: Extent3d,
    ) {
        let source_texture = self
            .pass_context
            .resource_table
            .get_resource(&source.texture);
        let destination_buffer = self
            .pass_context
            .resource_table
            .get_resource(&destination.buffer);

        self.pass_context.command_encoder.copy_texture_to_buffer(
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
        let buffer = self.pass_context.resource_table.get_resource(buffer_ref);

        self.pass_context
            .command_encoder
            .clear_buffer(&buffer.resource, offset, size);
    }

    pub fn clear_texture(
        &mut self,
        texture_ref: &Ref<TransientTexture, ResourceWrite>,
        subresource_range: &ImageSubresourceRange,
    ) {
        let texture = self.pass_context.resource_table.get_resource(texture_ref);

        self.pass_context
            .command_encoder
            .clear_texture(&texture.resource, subresource_range);
    }

    pub fn copy_texture_to_texture(
        &mut self,
        source: TexelCopyTextureInfo<ResourceRead>,
        destination: TexelCopyTextureInfo<ResourceWrite>,
        copy_size: Extent3d,
    ) {
        let source_texture = self
            .pass_context
            .resource_table
            .get_resource(&source.texture);
        let destination_texture = self
            .pass_context
            .resource_table
            .get_resource(&destination.texture);

        self.pass_context.command_encoder.copy_texture_to_texture(
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

    pub fn end_pipeline_statistics_query(&mut self) {
        self.render_pass
            .get_render_pass_mut()
            .end_pipeline_statistics_query();
    }

    pub fn begin_pipeline_statistics_query(&mut self, query_set: &QuerySet, index: u32) {
        self.render_pass
            .get_render_pass_mut()
            .begin_pipeline_statistics_query(query_set, index);
    }

    pub fn write_timestamp(&mut self, query_set: &QuerySet, index: u32) {
        self.render_pass
            .get_render_pass_mut()
            .write_timestamp(query_set, index);
    }

    pub fn set_blend_constant(&mut self, color: wgpu::Color) {
        self.render_pass
            .get_render_pass_mut()
            .set_blend_constant(color);
    }

    pub fn pop_debug_group(&mut self) {
        self.render_pass.get_render_pass_mut().pop_debug_group();
    }

    pub fn push_debug_group(&mut self, label: &str) {
        self.render_pass
            .get_render_pass_mut()
            .push_debug_group(label);
    }

    pub fn insert_debug_marker(&mut self, label: &str) {
        self.render_pass
            .get_render_pass_mut()
            .insert_debug_marker(label);
    }

    pub fn set_viewport(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
    ) {
        self.render_pass
            .get_render_pass_mut()
            .set_viewport(x, y, width, height, min_depth, max_depth);
    }

    pub fn set_push_constants(&mut self, stages: ShaderStages, offset: u32, data: &[u8]) {
        self.render_pass
            .get_render_pass_mut()
            .set_push_constants(stages, offset, data);
    }

    pub fn set_stencil_reference(&mut self, reference: u32) {
        self.render_pass
            .get_render_pass_mut()
            .set_stencil_reference(reference);
    }

    pub fn multi_draw_indexed_indirect_count(
        &mut self,
        indirect_buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        indirect_offset: u64,
        count_buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        count_offset: u64,
        max_count: u32,
    ) {
        let indirect_buffer = self
            .pass_context
            .resource_table
            .get_resource(indirect_buffer_ref);
        let count_buffer = self
            .pass_context
            .resource_table
            .get_resource(count_buffer_ref);

        self.render_pass
            .get_render_pass_mut()
            .multi_draw_indexed_indirect_count(
                &indirect_buffer.resource,
                indirect_offset,
                &count_buffer.resource,
                count_offset,
                max_count,
            );
    }

    pub fn multi_draw_indexed_indirect(
        &mut self,
        indirect_buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        indirect_offset: u64,
        count: u32,
    ) {
        let indirect_buffer = self
            .pass_context
            .resource_table
            .get_resource(indirect_buffer_ref);

        self.render_pass
            .get_render_pass_mut()
            .multi_draw_indexed_indirect(&indirect_buffer.resource, indirect_offset, count);
    }

    pub fn multi_draw_indirect_count(
        &mut self,
        indirect_buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        indirect_offset: u64,
        count_buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        count_offset: u64,
        max_count: u32,
    ) {
        let indirect_buffer = self
            .pass_context
            .resource_table
            .get_resource(indirect_buffer_ref);
        let count_buffer = self
            .pass_context
            .resource_table
            .get_resource(count_buffer_ref);

        self.render_pass
            .get_render_pass_mut()
            .multi_draw_indirect_count(
                &indirect_buffer.resource,
                indirect_offset,
                &count_buffer.resource,
                count_offset,
                max_count,
            );
    }

    pub fn multi_draw_indirect(
        &mut self,
        indirect_buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        indirect_offset: u64,
        count: u32,
    ) {
        let indirect_buffer = self
            .pass_context
            .resource_table
            .get_resource(indirect_buffer_ref);

        self.render_pass.get_render_pass_mut().multi_draw_indirect(
            &indirect_buffer.resource,
            indirect_offset,
            count,
        );
    }

    pub fn draw_indexed_indirect(
        &mut self,
        indirect_buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        indirect_offset: u64,
    ) {
        let indirect_buffer = self
            .pass_context
            .resource_table
            .get_resource(indirect_buffer_ref);

        self.render_pass
            .get_render_pass_mut()
            .draw_indexed_indirect(&indirect_buffer.resource, indirect_offset);
    }

    pub fn draw_indirect(
        &mut self,
        indirect_buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        indirect_offset: u64,
    ) {
        let indirect_buffer = self
            .pass_context
            .resource_table
            .get_resource(indirect_buffer_ref);

        self.render_pass
            .get_render_pass_mut()
            .draw_indirect(&indirect_buffer.resource, indirect_offset);
    }

    pub fn set_scissor_rect(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.render_pass
            .get_render_pass_mut()
            .set_scissor_rect(x, y, width, height);
    }

    pub fn set_bind_group(&mut self, index: u32, bind_group: &BindGroupInfo, offsets: &[u32]) {
        let bind_group = BindGroup::new(self.pass_context, bind_group);
        self.render_pass.get_render_pass_mut().set_bind_group(
            index,
            bind_group.get_bind_group(),
            offsets,
        );
    }

    pub fn set_render_pipeline(&mut self, id: CachedPipelineId) {
        let pipeline = self.pass_context.get_render_pipeline(id);
        self.render_pass
            .get_render_pass_mut()
            .set_pipeline(pipeline.wgpu());
    }

    pub fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        self.render_pass
            .get_render_pass_mut()
            .draw_indexed(indices, base_vertex, instances);
    }

    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.render_pass
            .get_render_pass_mut()
            .draw(vertices, instances);
    }

    pub fn set_vertex_buffer(
        &mut self,
        slot: u32,
        buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        offset: u64,
        size: u64,
    ) {
        let buffer = self.pass_context.resource_table.get_resource(buffer_ref);
        self.render_pass
            .get_render_pass_mut()
            .set_vertex_buffer(slot, buffer.resource.slice(offset..(offset + size)));
    }

    pub fn set_index_buffer(
        &mut self,
        buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        index_format: wgpu::IndexFormat,
        offset: u64,
        size: u64,
    ) {
        let buffer = self.pass_context.resource_table.get_resource(buffer_ref);

        self.render_pass
            .get_render_pass_mut()
            .set_index_buffer(buffer.resource.slice(offset..(offset + size)), index_format);
    }
}
