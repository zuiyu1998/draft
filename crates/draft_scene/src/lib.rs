use draft_geometry::{Circle, GeometryResource};
use draft_material::{Material, MaterialResource};
use draft_render::{GeometryInstanceData, RenderContext, World};
use fyrox_core::uuid;
use fyrox_resource::untyped::ResourceKind;

pub struct SceneContainer {
    geometry: GeometryResource,
    material: MaterialResource,
}

impl Default for SceneContainer {
    fn default() -> Self {
        let geometry = GeometryResource::new_ok(
            uuid!("33ee0142-f345-4c0a-9aca-d1f684a3485b"),
            ResourceKind::External,
            Circle::default().into(),
        );

        let material = MaterialResource::new_ok(
            uuid!("33ee0142-f345-4c0a-9aca-d1f684a34856"),
            ResourceKind::External,
            Material::default(),
        );

        SceneContainer { geometry, material }
    }
}

impl World for SceneContainer {
    fn prepare(&self, context: &mut RenderContext) {
        context.push(
            self.geometry.clone(),
            self.material.clone(),
            GeometryInstanceData {},
        );
    }
}
