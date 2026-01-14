struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4f,
};

struct Mesh2d {
    world_from_local: mat3x4<f32>,
};

@group(0) @binding(0) var<storage> mesh: array<Mesh2d>;

@vertex
fn vertex(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4f(model.position, 1.0);
    return out;
}

// 片元着色器

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4f {
    return vec4<f32>(1.0, 0.0, 1.0, 1.0);
}