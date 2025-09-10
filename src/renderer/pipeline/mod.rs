use draft_render::{
    FrameContext, FrameworkError, FrameworkErrorKind, GeometryResource, MaterialEffectContext,
    MaterialResource, PipelineDescriptor, RenderPhasesContainers, RenderWorld,
    frame_graph::{BufferInfo, TextureView},
    gfx_base::{
        BufferInitDescriptor, RenderDevice, VertexAttribute, VertexBufferLayout, VertexFormat,
        VertexStepMode,
    },
    render_resource::RenderBuffer,
    wgpu::BufferUsages,
};
use fxhash::FxHashMap;

use crate::{
    renderer::{MeshRenderPhase, MeshRenderPhaseExtractor, ObserversCollection, PhaseContext},
    scene::{DynSceneNode, Mesh},
};

#[derive(Clone)]
pub struct Batch {
    pub geometry: GeometryResource,
    pub material: MaterialResource,
    pub instance: GeometryInstance,
}

#[derive(Default, Clone)]
pub struct GeometryInstance {
    pub bytes: Vec<u8>,
    pub count: usize,
}

impl GeometryInstance {
    pub fn get_vertex_layout() -> VertexBufferLayout {
        VertexBufferLayout {
            array_stride: 64,
            step_mode: VertexStepMode::Instance,
            attributes: vec![
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 5,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 16,
                    shader_location: 6,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 32,
                    shader_location: 7,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 48,
                    shader_location: 8,
                },
            ],
        }
    }
}

pub struct InstanceRenderBuffer {
    pub buffer: RenderBuffer,
    pub count: usize,
}

fn get_instance_render_buffer_key(index: usize) -> String {
    format!("instance_render_buffer_{index}")
}

fn create_instance_render_buffer(
    device: &RenderDevice,
    instance_buffer_data: &[u8],
    index: usize,
    count: usize,
) -> InstanceRenderBuffer {
    let init_desc = BufferInitDescriptor {
        label: None,
        contents: instance_buffer_data,
        usage: BufferUsages::VERTEX,
    };

    let buffer = device.create_buffer_init(&init_desc);
    let buffer_desc = init_desc.to_buffer_desc();

    InstanceRenderBuffer {
        buffer: RenderBuffer {
            key: get_instance_render_buffer_key(index),
            value: buffer,
            desc: BufferInfo::from_buffer_desc(&buffer_desc),
        },
        count,
    }
}

impl Batch {
    pub fn new(geometry: GeometryResource, material: MaterialResource) -> Self {
        Self {
            geometry,
            material,
            instance: Default::default(),
        }
    }
}

#[derive(Default)]
struct BatchContainer {
    pub batches: FxHashMap<u64, Batch>,
}

impl DynRenderDataBundle for BatchContainer {}

impl MeshRenderPhaseExtractor for Batch {
    fn prepare(&self, context: PhaseContext) -> Result<MeshRenderPhase, FrameworkError> {
        let PhaseContext {
            render_world,
            frame_context,
            camera_offset,
            camera_size,
        } = context;

        let mut vertex_buffers = vec![];

        {
            let geometry = self.geometry.state();
            let Some(geometry) = geometry.data_ref() else {
                return Err(self.geometry.clone().into());
            };

            let vertex_buffer = geometry.vertex.get_vertex_layout();
            vertex_buffers.push(vertex_buffer);
            vertex_buffers.push(GeometryInstance::get_vertex_layout());
        }

        let geometry_data = render_world.geometry_cache.get_or_create(
            &render_world.server.device,
            &self.geometry,
            &vertex_buffers,
        )?;

        let vertex_buffer = geometry_data.get_vertex_buffer();
        let index_buffer = geometry_data.get_index_buffer();
        let instance_buffer = create_instance_render_buffer(
            &render_world.server.device,
            &self.instance.bytes,
            geometry_data.get_index(),
            self.instance.count,
        );

        let material = self.material.state();
        let Some(material) = material.data_ref() else {
            return Err(self.material.clone().into());
        };

        let mut layouts = vec![];
        let mut material_bind_group_handles = vec![];

        for effect_info in material.effect_infos().iter() {
            let effect = render_world
                .material_effect_container
                .get(&effect_info.effect_name)
                .ok_or(FrameworkErrorKind::MaterialEffectNotFound(
                    effect_info.effect_name.to_string(),
                ))?;

            layouts.push(effect.get_bind_group_layout_descriptor());

            let context = MaterialEffectContext {
                resource_bindings: &material.resource_bindings,
                frame_context,
                camera_offset,
                camera_size,
                world: render_world,
            };

            material_bind_group_handles.push(effect.process(context)?);
        }

        let desc = PipelineDescriptor::new(&material.pipeline_info, &layouts, &vertex_buffers);

        let pipeline_id = render_world.pipeline_cache.get_or_create(&desc);

        let mesh_phase = MeshRenderPhase {
            vertex_buffer,
            index_buffer,
            pipeline_id,
            material_bind_group_handles,
            instance_buffer,
        };

        Ok(mesh_phase)
    }
}

pub struct PipelineContext<'a> {
    pub mesh: &'a Mesh,
    pub texture_view: TextureView,
}

pub struct FrameRenderContext<'a> {
    pub render_data_bundle: &'a mut dyn DynRenderDataBundle,
    pub observers_collection: &'a mut ObserversCollection,
}

pub trait DynRenderDataBundle {}

impl PipelineContext<'_> {
    pub fn prepare(
        &self,
        observers_collection: &mut ObserversCollection,
        render_world: &mut RenderWorld,
    ) -> Result<FrameContext, FrameworkError> {
        let mut batch_container = BatchContainer::default();

        let mut render_context = FrameRenderContext {
            observers_collection,
            render_data_bundle: &mut batch_container,
        };

        self.mesh.collect_render_data(&mut render_context);

        let render_phases_containers =
            RenderPhasesContainers::alloc(observers_collection.cameras.len());

        let camera_uniforms = observers_collection.prepare(render_world);

        let mut frame_context = FrameContext::new(camera_uniforms, render_phases_containers);

        if let Some(ref camera_uniforms) = frame_context.camera_uniforms {
            for (camera, camera_offset) in camera_uniforms.camera_offsets.iter().enumerate() {
                for batch in batch_container.batches.values() {
                    let context = PhaseContext {
                        frame_context: &frame_context,
                        camera_offset: *camera_offset,
                        render_world,
                        camera_size: camera_uniforms.camera_size,
                    };
                    let render_phase = batch.prepare(context)?;

                    frame_context
                        .render_phases_containers
                        .camera_mut(camera)
                        .push(render_phase);
                }
            }
        }

        Ok(frame_context)
    }
}
