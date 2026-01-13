use draft_graphics::ShaderStages;
use draft_material::{
    MaterialBindGroup, MaterialBindGroupLayoutBuilder, MaterialEffect, MaterialEffectResource,
    binding_types::storage_buffer_read_only,
};
use draft_render::{IntoMeshMaterialInstanceData, MeshMaterialInstanceData};
use draft_shader::{Shader, ShaderResource};
use encase::{ShaderType, UniformBuffer};
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

#[derive(Debug, Clone, Default, ShaderType)]
pub struct Mesh2dUniform {
    pub world_from_local: [Vector4<f32>; 3],
}

impl IntoMeshMaterialInstanceData for Mesh2dUniform {
    fn into_mesh_material_instance_data(self) -> MeshMaterialInstanceData {
        let mut buffer = UniformBuffer::new(Vec::<u8>::new());
        buffer.write(&self).unwrap();
        let data = buffer.into_inner();
        MeshMaterialInstanceData { data }
    }
}

fn material_effect_2d() -> MaterialEffect {
    let mut effect = MaterialEffect::default();
    effect.name = "material_effect_2d".to_string();

    let mut bind_groups = vec![];

    let mesh2d_layout =
        MaterialBindGroupLayoutBuilder::new("mesh2d_layout", ShaderStages::VERTEX_FRAGMENT, 1)
            .with(
                "draft_mesh_2d",
                0,
                storage_buffer_read_only::<Mesh2dUniform>(false),
            )
            .build();

    bind_groups.push(MaterialBindGroup {
        name: "mesh2d".to_string(),
        layout: mesh2d_layout,
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
