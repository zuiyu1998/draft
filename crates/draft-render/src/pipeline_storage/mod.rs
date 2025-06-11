pub mod cache;
pub mod layout_cache;

pub use cache::*;
pub use layout_cache::*;

use std::{borrow::Cow, sync::Arc};

use frame_graph::{
    GetPipelineCache, Pipeline, PipelineCache, RenderDevice, RenderPipeline,
    wgpu::{
        self, FragmentState as RawFragmentState,
        RenderPipelineDescriptor as RawRenderPipelineDescriptor, ShaderModuleDescriptor,
        ShaderSource, VertexAttribute as RawVertexAttribute,
        VertexBufferLayout as RawVertexBufferLayout, VertexState as RawVertexState,
    },
};
use fyrox_core::log::Log;

use crate::{
    FrameworkError, MaterialResource, PipelineDescriptor, RenderPipelineDescriptor, Shader,
    ShaderResource,
};

pub struct ShaderModuleData {
    pub module: Arc<wgpu::ShaderModule>,
}

impl ShaderModuleData {
    pub fn new(
        composer: &mut naga_oil::compose::Composer,
        device: &RenderDevice,
        shader: &Shader,
    ) -> Result<Self, FrameworkError> {
        let naga = composer.make_naga_module(naga_oil::compose::NagaModuleDescriptor {
            ..(&shader.definition).into()
        })?;

        let shader_source = ShaderSource::Naga(Cow::Owned(naga));

        let module_descriptor = ShaderModuleDescriptor {
            label: None,
            source: shader_source,
        };

        let shader_module = device.wgpu_device().create_shader_module(module_descriptor);

        Ok(ShaderModuleData {
            module: Arc::new(shader_module),
        })
    }
}

#[derive(Default)]
pub struct ShaderCache {
    composer: naga_oil::compose::Composer,
    cache: TemporaryCache<ShaderModuleData>,
}

impl ShaderCache {
    pub fn get(
        &mut self,
        device: &RenderDevice,
        shader: &ShaderResource,
    ) -> Result<&wgpu::ShaderModule, FrameworkError> {
        let mut shader_state = shader.state();

        if let Some(shader_state) = shader_state.data() {
            match self.cache.get_or_insert_with(
                &shader_state.cache_index,
                Default::default(),
                || ShaderModuleData::new(&mut self.composer, device, shader_state),
            ) {
                Ok(data) => Ok(&data.module),
                Err(error) => Err(error),
            }
        } else {
            Err(FrameworkError::ShaderNotLoaded(shader.clone()))
        }
    }
}

pub struct MaterialData {
    pub pipeline: CachedPipeline,
    pub layout: wgpu::PipelineLayout,
}

impl MaterialData {
    pub fn get_render_pipeline_descriptor(
        shader_cache: &mut ShaderCache,
        pipeline_layout_cache: &mut PipelineLayoutCache,
        device: &RenderDevice,
        desc: &RenderPipelineDescriptor,
    ) -> Result<MaterialData, FrameworkError> {
        let vertex_module = shader_cache.get(device, &desc.vertex.shader)?.clone();
        let fragment_module = match &desc.fragment {
            Some(fragment) => match shader_cache.get(device, &fragment.shader) {
                Ok(module) => Some(module.clone()),
                Err(err) => return Err(err),
            },
            None => None,
        };

        let layout = pipeline_layout_cache.get(device, &desc.layout)?.clone();

        let vertex_buffer_layouts = desc
            .vertex
            .buffers
            .iter()
            .map(|layout| {
                (
                    layout.array_stride,
                    layout
                        .attributes
                        .iter()
                        .map(|attribute| attribute.into())
                        .collect::<Vec<RawVertexAttribute>>(),
                    layout.step_mode,
                )
            })
            .collect::<Vec<_>>();
        let vertex_buffer_layouts = vertex_buffer_layouts
            .iter()
            .map(
                |(array_stride, attributes, step_mode)| RawVertexBufferLayout {
                    array_stride: *array_stride,
                    attributes,
                    step_mode: (*step_mode).into(),
                },
            )
            .collect::<Vec<_>>();

        let fragment_data = desc.fragment.clone().map(|fragment| {
            (
                fragment_module.unwrap(),
                fragment.entry_point,
                fragment
                    .targets
                    .iter()
                    .map(|target| target.as_ref().map(|target| target.into()))
                    .collect::<Vec<_>>(),
                fragment.compilation_options,
            )
        });

        let descriptor = RawRenderPipelineDescriptor {
            multiview: None,
            depth_stencil: desc
                .depth_stencil
                .as_ref()
                .map(|depth_stencil| depth_stencil.into()),
            label: Some(&desc.label),
            layout: Some(&layout),
            multisample: desc.multisample.into(),
            primitive: desc.primitive.into(),
            vertex: RawVertexState {
                buffers: &vertex_buffer_layouts,
                entry_point: desc.vertex.entry_point.as_deref(),
                module: &vertex_module,
                compilation_options: desc.vertex.compilation_options.get_raw(),
            },
            fragment: fragment_data.as_ref().map(
                |(module, entry_point, targets, compilation_options)| RawFragmentState {
                    entry_point: entry_point.as_deref(),
                    module,
                    targets,
                    compilation_options: compilation_options.get_raw(),
                },
            ),
            cache: None,
        };

        let pipeline = device.wgpu_device().create_render_pipeline(&descriptor);

        Ok(MaterialData {
            pipeline: CachedPipeline {
                descriptor: PipelineDescriptor::RenderPipelineDescriptor(Box::new(desc.clone())),
                pipeline: Pipeline::RenderPipeline(RenderPipeline::new(pipeline)),
            },
            layout,
        })
    }

    pub fn new(
        shader_cache: &mut ShaderCache,
        pipeline_layout_cache: &mut PipelineLayoutCache,
        device: &RenderDevice,
        desc: &PipelineDescriptor,
    ) -> Result<Self, FrameworkError> {
        match &desc {
            PipelineDescriptor::RenderPipelineDescriptor(desc) => {
                Self::get_render_pipeline_descriptor(
                    shader_cache,
                    pipeline_layout_cache,
                    device,
                    desc,
                )
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

pub struct CachedPipeline {
    pub descriptor: PipelineDescriptor,
    pub pipeline: Pipeline,
}

#[derive(Default)]
pub struct PipelineStorage {
    pub shader_cache: ShaderCache,
    pub pipeline_layout_cache: PipelineLayoutCache,
    pub material_cache: TemporaryCache<MaterialData>,
}

impl PipelineStorage {
    pub fn get(
        &mut self,
        device: &RenderDevice,
        material: &MaterialResource,
    ) -> Option<&MaterialData> {
        let mut material_state = material.state();

        if let Some(material_state) = material_state.data() {
            match self.material_cache.get_or_insert_with(
                &material_state.cache_index,
                Default::default(),
                || {
                    MaterialData::new(
                        &mut self.shader_cache,
                        &mut self.pipeline_layout_cache,
                        device,
                        &material_state.desc,
                    )
                },
            ) {
                Ok(data) => Some(data),
                Err(error) => {
                    Log::err(format!("{error}"));
                    None
                }
            }
        } else {
            None
        }
    }
}

impl GetPipelineCache for PipelineStorage {
    fn get_pipeline_cache(&self) -> PipelineCache {
        let mut target = vec![];
        for index in 0..self.material_cache.buffer.len() {
            let pipeline = self
                .material_cache
                .buffer
                .get_raw(index)
                .map(|entry| entry.value.pipeline.pipeline.clone());

            target.push(pipeline);
        }

        PipelineCache::new(target)
    }
}
