use std::{borrow::Cow, collections::HashMap, sync::Arc};

use draft_graphics::{
    ColorTargetState, DepthStencilState, FragmentState, MultisampleState, Pipeline,
    PipelineCompilationOptions, PrimitiveState, RenderDevice, RenderPipelineDescriptor,
    ShaderModule, VertexBufferLayout as RawVertexBufferLayout, VertexState,
};
use draft_mesh::VertexBufferLayout;
use draft_shader::ShaderResource;
use wgpu::ShaderModuleDescriptor;

use crate::{
    FrameworkError,
    frame_graph::{GetPipelineContainer, PipelineContainer},
};

pub type CachePipelineId = usize;

pub struct GpuVertexState {
    pub shader: ShaderResource,
    pub entry_point: Option<Cow<'static, str>>,
    pub buffers: Vec<VertexBufferLayout>,
}

pub struct GpuFragmentState {
    pub shader: ShaderResource,
    pub entry_point: Option<Cow<'static, str>>,
    pub targets: Vec<Option<ColorTargetState>>,
}

pub struct GpuRenderPipelineDescriptor {
    pub label: String,
    pub vertex: GpuVertexState,
    pub primitive: PrimitiveState,
    pub depth_stencil: Option<DepthStencilState>,
    pub multisample: MultisampleState,
    pub fragment: Option<GpuFragmentState>,
    pub zero_initialize_workgroup_memory: bool,
}

pub struct PipelineCache {
    device: RenderDevice,
    shader_cache: HashMap<usize, Arc<ShaderModule>>,
    pipelines: Vec<Option<Pipeline>>,
}

impl GetPipelineContainer for PipelineCache {
    fn get_pipeline_container(&self) -> PipelineContainer {
        let mut pipelines = PipelineContainer::default();

        for pipeline in self.pipelines.iter() {
            pipelines.push(pipeline.as_ref().map(|pipeline| pipeline.clone()));
        }

        pipelines
    }
}

impl PipelineCache {
    pub fn new(device: &RenderDevice) -> Self {
        Self {
            device: device.clone(),
            shader_cache: HashMap::default(),
            pipelines: Default::default(),
        }
    }

    pub fn create_shader_moudule(&mut self, shader: &ShaderResource) -> Arc<ShaderModule> {
        let shader_data = shader.data_ref();

        let id = shader_data.cache_index.get();

        let shader_module = self.device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: shader_data.source.get_shader_source(),
        });

        let shader_module = Arc::new(shader_module);

        self.shader_cache.insert(id, shader_module.clone());

        shader_module
    }

    pub fn create_render_pipeline(
        &mut self,
        desc: &GpuRenderPipelineDescriptor,
    ) -> Result<CachePipelineId, FrameworkError> {
        let buffers = desc
            .vertex
            .buffers
            .iter()
            .map(|buffer| {
                (
                    buffer.array_stride,
                    buffer.step_mode,
                    buffer
                        .attributes
                        .iter()
                        .map(|attribute| attribute.get_wgpu_vertex_attribute())
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();

        let buffers = buffers
            .iter()
            .map(|buffer| RawVertexBufferLayout {
                array_stride: buffer.0,
                step_mode: buffer.1,
                attributes: &buffer.2,
            })
            .collect::<Vec<_>>();

        let vertext_state = VertexState {
            module: &self.create_shader_moudule(&desc.vertex.shader),
            entry_point: desc.vertex.entry_point.as_deref(),
            compilation_options: PipelineCompilationOptions {
                constants: &[],
                zero_initialize_workgroup_memory: desc.zero_initialize_workgroup_memory,
            },
            buffers: &buffers,
        };

        let fragment_state = match &desc.fragment {
            None => None,
            Some(frame_state) => Some(FragmentState {
                module: &self.create_shader_moudule(&frame_state.shader),
                entry_point: frame_state.entry_point.as_deref(),
                compilation_options: PipelineCompilationOptions {
                    constants: &[],
                    zero_initialize_workgroup_memory: desc.zero_initialize_workgroup_memory,
                },
                targets: &frame_state.targets,
            }),
        };

        let render_pipeline = self
            .device
            .create_render_pipelie(&RenderPipelineDescriptor {
                vertex: vertext_state,
                fragment: fragment_state,
                label: Some(&desc.label),
                layout: None,
                primitive: desc.primitive,
                multisample: desc.multisample,
                depth_stencil: desc.depth_stencil.clone(),
                multiview_mask: None,
                cache: None,
            });

        let len = self.pipelines.len();

        self.pipelines
            .push(Some(Pipeline::RenderPipeline(render_pipeline)));

        Ok(len)
    }
}
