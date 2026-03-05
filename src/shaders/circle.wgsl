struct Uniforms {
    screen: vec2<f32>,  // viewport width, height in pixels
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

fn to_clip(pixel: vec2<f32>) -> vec2<f32> {
    // Pixel (0,0) is top-left
    // Clip (-1,1) is top-left
    return vec2(
         2.0 * pixel.x / uniforms.screen.x - 1.0,
        -2.0 * pixel.y / uniforms.screen.y + 1.0,
    );
}

// Thickness grows border outwards
struct InstanceInput {
    @location(0) center: vec2<f32>,
    @location(1) radius: f32,
    @location(2) thickness: f32,
    @location(3) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) center: vec2<f32>,
    @location(2) radius: f32,
    @location(3) thickness: f32,
    @location(4) color: vec4<f32>,
};

fn sdCircle(p: vec2<f32>, r: f32) -> f32 {
    return length(p) - r;
}

@vertex
fn vs_main(
    @builtin(vertex_index) vid: u32,
    instance: InstanceInput,
) -> VertexOutput {
    // Pad the bounding quad by thickness so the border isn't clipped at the edge
    let extent = instance.radius + instance.thickness;

    // Bit 0 → left/right, Bit 1 → top/bottom
    let x = instance.center.x + select(-extent, extent, (vid & 1u) != 0u);
    let y = instance.center.y + select(-extent, extent, (vid & 2u) != 0u);

    var out: VertexOutput;
    out.clip_position = vec4<f32>(to_clip(vec2(x, y)), 0.0, 1.0);
    out.center    = instance.center;
    out.radius    = instance.radius;
    out.thickness = instance.thickness;
    out.color     = instance.color;
    return out;
}

// Fragment shader

@fragment
fn fs_border(in: VertexOutput) -> @location(0) vec4<f32> {
    var sdf = sdCircle(in.clip_position.xy - in.center, in.radius);
    var mask = abs(sdf - in.thickness / 2.0) - in.thickness / 2.0;
    let alpha = clamp(-mask, 0.0, 1.0);
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}

@fragment
fn fs_fill(in: VertexOutput) -> @location(0) vec4<f32> {
    let d = sdCircle(in.clip_position.xy - in.center, in.radius);
    let alpha = clamp(-d, 0.0, 1.0);
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
