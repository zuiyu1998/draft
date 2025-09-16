use draft_render::{
    FrameContext, GeometryResource, MaterialResource, MeshInstanceData, RenderDataBundleStorage,
    RenderWorld,
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
    renderer::ObserversCollection,
    scene::{DynSceneNode, Mesh, NodeContext},
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

impl RenderDataBundleStorage for BatchContainer {
    fn push_mesh(
        &mut self,
        _geometry: GeometryResource,
        _material: MaterialResource,
        _sort_index: u64,
        _instance_data: MeshInstanceData,
    ) {
        todo!()
    }
}

pub struct PipelineContext<'a> {
    pub mesh: &'a Mesh,
    pub texture_view: TextureView,
}

impl PipelineContext<'_> {
    pub fn prepare(
        &self,
        observers_collection: &mut ObserversCollection,
        render_world: &mut RenderWorld,
    ) -> Option<FrameContext> {
        let mut batch_container = BatchContainer::default();

        let mut node_context: NodeContext<'_> = NodeContext {
            observers_collection,
            render_data_bundle_storage: &mut batch_container,
        };

        self.mesh.collect_render_data(&mut node_context);

        observers_collection
            .prepare(render_world)
            .map(|camera_uniforms| FrameContext::new(camera_uniforms, batch_container))
    }
}
