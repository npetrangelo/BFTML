struct VertexInput {
    @location(0) position: vec3<f32>,
};

// Thickness grows border outwards
struct InstanceInput {
    @location(1) center: vec2<f32>,
    @location(2) radius: f32,
    @location(3) thickness: f32,
    @location(4) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) center: vec2<f32>,
    @location(1) radius: f32,
    @location(2) thickness: f32,
    @location(3) color: vec3<f32>,
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
    out.thickness = instance.thickness;
    out.color = instance.color;
    return out;
}

// Fragment shader

@fragment
fn fs_border(in: VertexOutput) -> @location(0) vec4<f32> {
    var sdf = sdCircle(in.clip_position.xy - in.center, in.radius);
    var mask = abs(sdf - in.thickness/2.0) - in.thickness/2.0;
    var pixel = -mask * in.color;
    return vec4<f32>(pixel, 1.0);
}

@fragment
fn fs_fill(in: VertexOutput) -> @location(0) vec4<f32> {
    var d = sdCircle(in.clip_position.xy - in.center, in.radius);
    var pixel: vec3<f32> = -d * in.color;
    return vec4<f32>(pixel, 1.0);
}
