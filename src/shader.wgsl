struct VertexInput {
    @location(0) position: vec2<f32>,
};

struct InstanceInput {
    @location(2) offset: vec2<f32>,
    @location(3) colour: vec3<f32>,
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
    out.color = vec3<f32>(instance.colour.x, instance.colour.y, instance.colour.z);
    out.clip_position = vec4<f32>(model.position.x+instance.offset.x, model.position.y+instance.offset.y, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}