use draft_mesh::{Circle, MeshResource};
use draft_material::{Material, MaterialResource};
use draft_render::{MeshInstanceData, RenderContext, World};
use draft_render_2d::Material2d;
use fyrox_core::uuid;
use fyrox_resource::untyped::ResourceKind;

pub struct SceneContainer {
    mesh: MeshResource,
    material: MaterialResource,
}

impl Default for SceneContainer {
    fn default() -> Self {
        let mesh = MeshResource::new_ok(
            uuid!("33ee0142-f345-4c0a-9aca-d1f684a3485b"),
            ResourceKind::External,
            Circle::default().into(),
        );

        let material = MaterialResource::new_ok(
            uuid!("33ee0142-f345-4c0a-9aca-d1f684a34856"),
            ResourceKind::External,
            Material::new::<Material2d>(),
        );

        SceneContainer { mesh, material }
    }
}

impl World for SceneContainer {
    fn prepare(&self, context: &mut RenderContext) {
        context.push(
            self.mesh.clone(),
            self.material.clone(),
            MeshInstanceData {},
        );
    }
}
