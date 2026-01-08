use draft_material::{MaterialBindGroup, MaterialEffect, MaterialEffectResource};
use draft_render::{IntoMeshMaterialInstanceData, MeshMaterialInstanceData};
use draft_shader::{Shader, ShaderResource};
use fyrox_core::{algebra::Vector4, uuid};
use fyrox_resource::{embedded_data_source, manager::BuiltInResource, untyped::ResourceKind};
use std::sync::LazyLock;

pub static MESH_2D: LazyLock<BuiltInResource<Shader>> = LazyLock::new(|| {
    BuiltInResource::new(
        "__MESH_2D__",
        embedded_data_source!("./mesh2d.wgsl"),
        |data| {
            ShaderResource::new_ok(
                uuid!("f5b02124-9601-452a-9368-3fa2a9703ecd"),
                ResourceKind::External,
                Shader::from_wgsl(String::from_utf8(data.to_vec()).unwrap(), ""),
            )
        },
    )
});

#[derive(Debug, Clone, Default)]
pub struct Mesh2dUniform {
    pub world_from_local: [Vector4<f32>; 3],
}

impl IntoMeshMaterialInstanceData for Mesh2dUniform {
    fn into_mesh_material_instance_data(self) -> MeshMaterialInstanceData {
        todo!()
    }
}

fn material_effect_2d() -> MaterialEffect {
    let mut effect = MaterialEffect::default();
    effect.name = "material_effect_2d".to_string();

    let mut bind_groups = vec![];

    // let d = MaterialBindGroupLayoutBuilder::new("mesh2d_layout", ShaderStages::VERTEX_FRAGMENT, 1)
    //     .with("draft_mesh_2d", 0, value);

    bind_groups.push(MaterialBindGroup {
        name: "mesh2d_layout".to_string(),
        layouts: vec![],
    });

    effect.bind_groups = bind_groups;

    effect
}

pub static MATERIAL_EFFECT_2D: LazyLock<BuiltInResource<MaterialEffect>> = LazyLock::new(|| {
    BuiltInResource::new_no_source(
        "__MATERIAL_EFFECT_2D__",
        MaterialEffectResource::new_ok(
            uuid!("b9deeb8e-0c43-456f-85d1-8f1a25983e75"),
            ResourceKind::External,
            material_effect_2d(),
        ),
    )
});
