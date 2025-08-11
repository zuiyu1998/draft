use std::ops::Range;

use wgpu::{Extent3d, ImageSubresourceRange, QuerySet, ShaderStages};

use crate::{
    frame_graph::{
        BindGroupInfo, Ref, ResourceRead, ResourceWrite, TexelCopyBufferInfo, TexelCopyTextureInfo,
        TransientBuffer, TransientTexture,
    },
    gfx_base::CachedPipelineId,
};

use super::{RenderPassCommand, RenderPassContext};

pub struct DispatchWorkgroupsParameter {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

pub struct ClearBufferParameter {
    pub buffer_ref: Ref<TransientBuffer, ResourceWrite>,
    pub offset: u64,
    pub size: Option<u64>,
}

impl RenderPassCommand for ClearBufferParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.clear_buffer(&self.buffer_ref, self.offset, self.size);
    }
}

pub struct ClearTextureParameter {
    pub texture_ref: Ref<TransientTexture, ResourceWrite>,
    pub subresource_range: ImageSubresourceRange,
}

impl RenderPassCommand for ClearTextureParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.clear_texture(&self.texture_ref, &self.subresource_range);
    }
}

pub struct CopyTextureToBufferParameter {
    pub source: TexelCopyTextureInfo<ResourceRead>,
    pub destination: TexelCopyBufferInfo<ResourceWrite>,
    pub copy_size: Extent3d,
}

impl RenderPassCommand for CopyTextureToBufferParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.copy_texture_to_buffer(
            self.source.clone(),
            self.destination.clone(),
            self.copy_size,
        );
    }
}

pub struct CopyTextureToTextureParameter {
    pub source: TexelCopyTextureInfo<ResourceRead>,
    pub destination: TexelCopyTextureInfo<ResourceWrite>,
    pub copy_size: Extent3d,
}

impl RenderPassCommand for CopyTextureToTextureParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.copy_texture_to_texture(
            self.source.clone(),
            self.destination.clone(),
            self.copy_size,
        );
    }
}

pub struct DispatchWorkgroupsIndirectParameter {
    pub indirect_buffer_ref: Ref<TransientBuffer, ResourceRead>,
    pub indirect_offset: u64,
}

pub struct DrawIndexedIndirectParameter {
    pub indirect_buffer_ref: Ref<TransientBuffer, ResourceRead>,
    pub indirect_offset: u64,
}

impl RenderPassCommand for DrawIndexedIndirectParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.draw_indexed_indirect(&self.indirect_buffer_ref, self.indirect_offset);
    }
}

pub struct MultiDrawIndirectParameter {
    pub indirect_buffer_ref: Ref<TransientBuffer, ResourceRead>,
    pub indirect_offset: u64,
    pub count: u32,
}

impl RenderPassCommand for MultiDrawIndirectParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.multi_draw_indirect(
            &self.indirect_buffer_ref,
            self.indirect_offset,
            self.count,
        );
    }
}

pub struct MultiDrawIndirectCountParameter {
    pub indirect_buffer_ref: Ref<TransientBuffer, ResourceRead>,
    pub indirect_offset: u64,
    pub count_buffer_ref: Ref<TransientBuffer, ResourceRead>,
    pub count_offset: u64,
    pub max_count: u32,
}

impl RenderPassCommand for MultiDrawIndirectCountParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.multi_draw_indexed_indirect_count(
            &self.indirect_buffer_ref,
            self.indirect_offset,
            &self.count_buffer_ref,
            self.count_offset,
            self.max_count,
        );
    }
}

pub struct MultiDrawIndexedIndirectParameter {
    pub indirect_buffer_ref: Ref<TransientBuffer, ResourceRead>,
    pub indirect_offset: u64,
    pub count: u32,
}

impl RenderPassCommand for MultiDrawIndexedIndirectParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.multi_draw_indexed_indirect(
            &self.indirect_buffer_ref,
            self.indirect_offset,
            self.count,
        );
    }
}

pub struct MultiDrawIndexedIndirectCountParameter {
    pub indirect_buffer_ref: Ref<TransientBuffer, ResourceRead>,
    pub indirect_offset: u64,
    pub count_buffer_ref: Ref<TransientBuffer, ResourceRead>,
    pub count_offset: u64,
    pub max_count: u32,
}

impl RenderPassCommand for MultiDrawIndexedIndirectCountParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.multi_draw_indexed_indirect_count(
            &self.indirect_buffer_ref,
            self.indirect_offset,
            &self.count_buffer_ref,
            self.count_offset,
            self.max_count,
        );
    }
}

pub struct SetStencilReferenceParameter {
    pub reference: u32,
}

impl RenderPassCommand for SetStencilReferenceParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.set_stencil_reference(self.reference);
    }
}

pub struct SetPushConstantsParameter {
    pub stages: ShaderStages,
    pub offset: u32,
    pub data: Vec<u8>,
}

pub struct SetPushConstantsComputeParameter {
    pub offset: u32,
    pub data: Vec<u8>,
}

impl RenderPassCommand for SetPushConstantsParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.set_push_constants(self.stages, self.offset, &self.data);
    }
}

pub struct SetViewportParameter {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

impl RenderPassCommand for SetViewportParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.set_viewport(
            self.x,
            self.y,
            self.width,
            self.height,
            self.min_depth,
            self.max_depth,
        );
    }
}

pub struct InsertDebugMarkerParameter {
    pub label: String,
}

impl RenderPassCommand for InsertDebugMarkerParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.insert_debug_marker(&self.label);
    }
}

pub struct PushDebugGroupParameter {
    pub label: String,
}

impl RenderPassCommand for PushDebugGroupParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.push_debug_group(&self.label);
    }
}

pub struct PopDebugGroupParameter;

impl RenderPassCommand for PopDebugGroupParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.pop_debug_group();
    }
}

pub struct SetBlendConstantParameter {
    pub color: wgpu::Color,
}

impl RenderPassCommand for SetBlendConstantParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.set_blend_constant(self.color);
    }
}

pub struct WriteTimestampParameter {
    pub query_set: QuerySet,
    pub index: u32,
}

impl RenderPassCommand for WriteTimestampParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.write_timestamp(&self.query_set, self.index);
    }
}

pub struct BeginPipelineStatisticsQueryParameter {
    pub query_set: QuerySet,
    pub index: u32,
}

impl RenderPassCommand for BeginPipelineStatisticsQueryParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.begin_pipeline_statistics_query(&self.query_set, self.index);
    }
}

pub struct EndPipelineStatisticsQueryParameter;

impl RenderPassCommand for EndPipelineStatisticsQueryParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.end_pipeline_statistics_query();
    }
}

pub struct DrawIndirectParameter {
    pub indirect_buffer_ref: Ref<TransientBuffer, ResourceRead>,
    pub indirect_offset: u64,
}

impl RenderPassCommand for DrawIndirectParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.draw_indirect(&self.indirect_buffer_ref, self.indirect_offset);
    }
}

pub struct DrawIndexedParameter {
    pub indices: Range<u32>,
    pub base_vertex: i32,
    pub instances: Range<u32>,
}

impl RenderPassCommand for DrawIndexedParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.draw_indexed(
            self.indices.clone(),
            self.base_vertex,
            self.instances.clone(),
        );
    }
}

pub struct SetScissorRectParameter {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl RenderPassCommand for SetScissorRectParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.set_scissor_rect(self.x, self.y, self.width, self.height);
    }
}

pub struct DrawParameter {
    pub vertices: Range<u32>,
    pub instances: Range<u32>,
}

impl RenderPassCommand for DrawParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.draw(self.vertices.clone(), self.instances.clone());
    }
}

pub struct SetIndexBufferParameter {
    pub buffer_ref: Ref<TransientBuffer, ResourceRead>,
    pub index_format: wgpu::IndexFormat,
    pub offset: u64,
    pub size: u64,
}

impl RenderPassCommand for SetIndexBufferParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.set_index_buffer(
            &self.buffer_ref,
            self.index_format,
            self.offset,
            self.size,
        );
    }
}

pub struct SetVertexBufferParameter {
    pub slot: u32,
    pub buffer_ref: Ref<TransientBuffer, ResourceRead>,
    pub offset: u64,
    pub size: u64,
}

impl RenderPassCommand for SetVertexBufferParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.set_vertex_buffer(self.slot, &self.buffer_ref, self.offset, self.size);
    }
}

pub struct SetComputePipelineParameter {
    pub id: CachedPipelineId,
}

pub struct SetRenderPipelineParameter {
    pub id: CachedPipelineId,
}

impl RenderPassCommand for SetRenderPipelineParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.set_render_pipeline(self.id);
    }
}

pub struct SetBindGroupParameter {
    pub index: u32,
    pub bind_group: BindGroupInfo,
    pub offsets: Vec<u32>,
}

impl RenderPassCommand for SetBindGroupParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.set_bind_group(self.index, &self.bind_group, &self.offsets);
    }
}
