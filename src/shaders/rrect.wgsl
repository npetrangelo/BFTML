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

struct VertexInput {
    @location(0) position: vec3<f32>,
};

// Thickness grows border outwards
struct InstanceInput {
    @location(0) left: f32,
    @location(1) right: f32,
    @location(2) top: f32,
    @location(3) bottom: f32,
    @location(4) thickness: f32,
    @location(5) radius: f32,
    @location(6) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) center: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) thickness: f32,
    @location(3) radius: f32,
    @location(4) color: vec4<f32>,
};

fn sdBox(p: vec2<f32>, b: vec2<f32>) -> f32 {
    var d = abs(p) - b;
    var outside = length(max(d, vec2(0.0)));
    var inside = min(max(d.x, d.y), 0.0);
    return outside + inside;
}

@vertex
fn vs_main(
    @builtin(vertex_index) vid: u32,
    instance: InstanceInput,
) -> VertexOutput {
    // Add thickness padding so the border isn't clipped at the quad edge
    let padding = instance.thickness;
    let left   = instance.left   - padding;
    let right  = instance.right  + padding;
    let top    = instance.top    - padding;
    let bottom = instance.bottom + padding;

    let x = select(left, right, (vid & 1u) != 0u);
    let y = select(top, bottom, (vid & 2u) != 0u);

    var out: VertexOutput;
    out.clip_position = vec4<f32>(to_clip(vec2(x, y)), 0.0, 1.0);
    
    // Pass through everything the fragment shader needs
    out.center = vec2(
        (instance.left + instance.right)  / 2.0,
        (instance.top  + instance.bottom) / 2.0,
    );
    out.size      = vec2(instance.right - instance.left, instance.bottom - instance.top);
    out.thickness = instance.thickness;
    out.color     = instance.color;
    return out;
}

// Fragment shader
// Rounded rectangle is a smaller rectangle farther out in its SDF
@fragment
fn fs_border(in: VertexOutput) -> @location(0) vec4<f32> {
    var size = in.size / 2.0 - in.radius;
    var sdf = sdBox(in.clip_position.xy - in.center, size) - in.radius;
    var mask = abs(sdf - in.thickness / 2.0) - in.thickness / 2.0;
    let alpha = clamp(-mask, 0.0, 1.0);
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
