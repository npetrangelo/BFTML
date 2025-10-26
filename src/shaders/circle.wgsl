struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct InstanceInput {
    @location(1) center: vec2<f32>,
    @location(2) radius: f32,
    @location(3) color: vec3<f32>,
    @location(4) thickness: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) center: vec2<f32>,
    @location(1) radius: f32,
    @location(2) color: vec3<f32>,
    @location(3) thickness: f32,
};

fn sdCircle(p: vec2<f32>, r: f32) -> f32 {
    return length(p) - r;
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.center = instance.center;
    out.radius = instance.radius;
    out.color = instance.color;
    out.thickness = instance.thickness;
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var d = sdCircle(in.clip_position.xy - in.center, in.radius);
    var pixel: vec3<f32> = in.color * (in.thickness - abs(d));
    return vec4<f32>(pixel, 1.0);
}
