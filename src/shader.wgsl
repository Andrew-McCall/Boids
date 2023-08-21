struct VertexInput {
    @location(0) position: vec3<f32>,
};
struct InstanceInput {
    @location(8) x_offset: f32,
    @location(9) y_offset: f32,
    @location(7) colour_r: f32,
    @location(5) colour_g: f32,
    @location(6) colour_b: f32,
};
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = vec3<f32>(instance.colour_r, instance.colour_g, instance.colour_b);
    out.clip_position = vec4<f32>(model.position.x+instance.x_offset, model.position.y+instance.y_offset, model.position.z, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}