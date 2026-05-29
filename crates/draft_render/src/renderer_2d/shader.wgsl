// struct Mesh2d {
// };

// @group(0) @binding(0) var<uniform> mesh: array<Mesh2d, 1024>;


struct VertexOutput {
    @builtin(position) clip_position: vec4f,
};

struct VertexInput {
    @location(0) position: vec3f,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4f(model.position, 1.0);
    return out;
}

// 片元着色器

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    return vec4f(0.3, 0.2, 0.1, 1.0);
}