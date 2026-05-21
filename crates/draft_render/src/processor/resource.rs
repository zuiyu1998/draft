use std::sync::LazyLock;

use draft_shader::{Shader, ShaderResource};
use fyrox_resource::{
    core::uuid, embedded_data_source, manager::BuiltInResource, untyped::ResourceKind,
};

pub static SHADER: LazyLock<BuiltInResource<Shader>> = LazyLock::new(|| {
    BuiltInResource::new("Shader", embedded_data_source!("shader.wgsl"), |data| {
        ShaderResource::new_ok(
            uuid!("33ee0142-f345-4c0a-9aca-d1f684a3485b"),
            ResourceKind::External,
            Shader::from_string_bytes(data),
        )
    })
});
