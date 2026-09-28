struct Uniforms {
    view_projection: mat4x4<f32>,
    model: mat4x4<f32>,
    options: vec4<f32>,
};
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
};
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) lit: f32,
};

@vertex fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world = uniforms.model * vec4<f32>(input.position, 1.0);
    out.position = uniforms.view_projection * world;
    out.world = world.xyz;
    out.color = input.color;
    out.lit = 0.0;
    if dot(input.normal, input.normal) > 0.5 {
        let normal = normalize((uniforms.model * vec4<f32>(input.normal, 0.0)).xyz);
        let light = normalize(vec3<f32>(0.4, 0.8, 0.6));
        out.color *= 0.28 + 0.72 * max(dot(normal, light), 0.0);
        out.lit = 1.0;
    }
    return out;
}

@fragment fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    var color = input.color;
    if uniforms.options.x > 0.5 && input.lit > 0.5 {
        let cell = floor(input.world * 3.0);
        let parity = cell.x + cell.y + cell.z;
        if (parity - 2.0 * floor(parity / 2.0)) > 0.5 {
            color *= 0.38;
        }
    }
    return vec4<f32>(color, 1.0);
}
