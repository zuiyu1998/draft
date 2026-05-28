use draft::{
    DefaultPlugins,
    app::App,
    core::Uuid,
    mesh::{IndexBuffer, Mesh, MeshResource, VertexAttributeValues},
    render::{IWorld, RenderContext},
    resource::{Resource, untyped::ResourceKind},
};

pub struct SceneTree {
    mesh: MeshResource,
}

impl SceneTree {
    pub fn new() -> SceneTree {
        let mut mesh = Mesh::default();

        mesh.vertex_buffer.get_mut().insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(vec![
                [0.0, 0.5, 0.0],
                [-0.5, -0.5, 0.0],
                [0.5, -0.5, 0.0],
            ]),
        );

        let mut index_buffer = IndexBuffer::default();
        index_buffer.get_mut().set_indices_with_u16(&[0, 1, 2]);
        mesh.index_buffer = Some(index_buffer);

        SceneTree {
            mesh: Resource::new_ok(Uuid::new_v4(), ResourceKind::External, mesh),
        }
    }
}

impl IWorld for SceneTree {
    fn render(&self, context: &mut RenderContext) {
        let mesh_id = context
            .render_world()
            .get_or_create_mesh_id(&self.mesh)
            .expect("get_or_create_mesh_id failed");

        let _pipeline_id = context
            .create_2d_render_pipeline(mesh_id)
            .expect("create_2d_render_pipeline failed");
    }
}

fn main() {
    let mut app = App::new();

    app.set_world(SceneTree::new());
    app.add_plugin(DefaultPlugins);

    app.run();
}
