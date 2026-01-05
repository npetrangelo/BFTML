struct VertexInput {
    @location(0) position: vec3<f32>,
};

// Thickness grows border outwards
struct InstanceInput {
    @location(1) left: f32,
    @location(2) right: f32,
    @location(3) top: f32,
    @location(4) bottom: f32,
    @location(5) thickness: f32,
    @location(6) radius: f32,
    @location(7) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) center: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) thickness: f32,
    @location(3) radius: f32,
    @location(4) color: vec3<f32>,
};

fn sdBox(p: vec2<f32>, b: vec2<f32>) -> f32 {
    var d = abs(p) - b;
    var outside = length(max(d, vec2(0.0)));
    var inside = min(max(d.x, d.y), 0.0);
    return outside + inside;
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.center = (vec2(instance.left, instance.top) + vec2(instance.right, instance.bottom))/2.0;
    out.size = vec2(instance.right - instance.left, instance.bottom - instance.top);
    out.thickness = instance.thickness;
    out.radius = instance.radius;
    out.color = instance.color;
    return out;
}

// Fragment shader
// Rounded rectangle is a smaller rectangle farther out in its SDF
@fragment
fn fs_border(in: VertexOutput) -> @location(0) vec4<f32> {
    var size = in.size - in.radius;
    var sdf = sdBox(in.clip_position.xy - in.center, size) - in.radius;
    var mask = abs(sdf - in.thickness/2.0) - in.thickness/2.0;
    return vec4<f32>(in.color, -mask);
}
